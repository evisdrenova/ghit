use crate::config::{Config, MessageLevel};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

struct LLM {
    config: Config,
}

impl LLM {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn gen_commit_message(&self, diff: &str) -> Result<(String, Option<String>)> {
        let message_level = &self.config.message_level;

        let style = match message_level {
            MessageLevel::Quiet => "a very brief, one-line",
            MessageLevel::Normal => "a concise subject plus short body",
            MessageLevel::Verbose => "a detailed subject and explanatory body",
        };

        let prompt = format!(
            "Write {} Git commit message for these staged changes. Follow conventional commit format.\n\nChanges:\n{}",
            style, diff
        );

        let response = self.call_openai_api(&prompt).await?;

        self.parse_commit_message(&response)
    }

    async fn call_openai_api(&self, prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a helpful assistant that writes clear, concise Git commit messages following conventional commit format.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            max_tokens: 200,
            temperature: 0.3,
        };

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        chat_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .context("No response from OpenAI")
    }

    fn parse_commit_message(&self, response: &str) -> Result<(String, Option<String>)> {
        let trimmed = response.trim();

        // Split on first empty line to separate subject from body
        if let Some((subject, body)) = trimmed.split_once("\n\n") {
            let subject = subject.lines().next().unwrap_or(subject).to_string();
            let body = body.trim();

            if body.is_empty() {
                Ok((subject, None))
            } else {
                Ok((subject, Some(body.to_string())))
            }
        } else {
            let subject = trimmed.lines().next().unwrap_or(trimmed).to_string();
            Ok((subject, None))
        }
    }
}

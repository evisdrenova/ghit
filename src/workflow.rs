use crate::{config::Config, git::Git, llm::LLM};
use anyhow::{Context, Result};

pub struct Workflow {
    llm: LLM,
    config: Config,
}

impl Workflow {
    pub fn new(config: Config) -> Self {
        let llm = LLM::new(config.clone());
        Self { llm, config }
    }

    /// Complete workflow: add files, generate commit message, commit, and push
    pub async fn auto_commit_and_push(&self, files: Vec<String>) -> Result<()> {
        println!("🔄 Starting automated commit workflow...");

        // Step 1: Add files to staging
        if !files.is_empty() {
            println!("📁 Adding files to staging area...");
            Git::add_files(&files).context("Failed to add files to staging area")?;
        }

        // Step 2: Check if there are staged changes
        if !Git::has_staged_changes()? {
            return Err(anyhow::anyhow!(
                "No staged changes found. Please add files with 'git add' or specify files to add."
            ));
        }

        // Step 3: Get the diff
        println!("📊 Reading staged changes...");
        let diff = Git::get_staged_diff().context("Failed to get staged diff")?;

        println!("🤖 Generating commit message with AI...");

        // Step 4: Generate commit message
        let (subject, body) = self
            .llm
            .gen_commit_message(&diff)
            .await
            .context("Failed to generate commit message")?;

        // Step 5: Show the generated message and confirm
        println!("\n📝 Generated commit message:");
        println!("Subject: {}", subject);
        if let Some(body_text) = &body {
            println!("Body:\n{}", body_text);
        }

        if !self.confirm_commit()? {
            println!("❌ Commit cancelled by user");
            return Ok(());
        }

        // Step 6: Create the commit
        println!("💾 Creating commit...");
        Git::commit(&subject, body.as_deref()).context("Failed to create commit")?;

        // Step 7: Push to remote
        println!("🚀 Pushing to remote...");
        let current_branch =
            Git::get_current_branch().unwrap_or_else(|_| self.config.default_branch.clone());

        Git::push(Some(&current_branch)).context("Failed to push to remote")?;

        println!("✅ Workflow completed successfully!");
        Ok(())
    }

    /// Just generate a commit message without committing
    pub async fn generate_message_only(&self) -> Result<()> {
        if !Git::has_staged_changes()? {
            return Err(anyhow::anyhow!(
                "No staged changes found. Please add files with 'git add' first."
            ));
        }

        println!("📊 Reading staged changes...");
        let diff = Git::get_staged_diff()?;

        println!("🤖 Generating commit message...");
        let (subject, body) = self.llm.gen_commit_message(&diff).await?;

        println!("\n📝 Generated commit message:");
        println!("Subject: {}", subject);
        if let Some(ref body_text) = body {
            // Add 'ref' here
            println!("Body:\n{}", body_text);
        }

        if !self.confirm_commit()? {
            println!("❌ Commit cancelled by user");
            return Ok(());
        }

        // Step 6: Create the commit
        println!("💾 Creating commit...");
        Git::commit(&subject, body.as_deref()) // Now this works
            .context("Failed to create commit")?;

        Ok(())
    }

    /// Add files and generate message, but don't commit
    pub async fn stage_and_generate(&self, files: Vec<String>) -> Result<()> {
        if !files.is_empty() {
            println!("📁 Adding files to staging area...");
            Git::add_files(&files)?;
        }

        self.generate_message_only().await
    }

    fn confirm_commit(&self) -> Result<bool> {
        use std::io::{self, Write};

        print!("\n❓ Create this commit? [Y/n]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        Ok(input.is_empty() || input == "y" || input == "yes")
    }
}

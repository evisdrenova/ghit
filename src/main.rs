use clap::Parser;

mod config;
mod git;
mod llm;
mod workflow;

use anyhow::Result;
use config::Config;
use workflow::Workflow;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    dry_run: bool,

    #[arg(short, long)]
    branch: Option<String>,

    /// Add files, generate message, commit, and push automatically
    #[arg(short, long)]
    auto: bool,

    /// Only generate commit message for staged changes
    #[arg(short, long)]
    generate: bool,

    /// Add files and generate message (don't commit)
    #[arg(short = 's', long)]
    stage_and_generate: bool,

    /// Files to add to staging
    files: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config::load().map_err(|e| {
        eprintln!("❌ Failed to load config: {}", e);
        eprintln!("Please create a ghit.toml file with:");
        eprintln!("model = \"gpt-3.5-turbo\"");
        eprintln!("api_key = \"your-openai-api-key\"");
        eprintln!("default_branch = \"main\"");
        eprintln!("message_level = \"normal\"  # quiet, normal, or verbose");
        e
    })?;

    let workflow = Workflow::new(config);
    if args.auto {
        workflow.auto_commit_and_push(args.files).await?;
    } else if args.generate {
        if !args.files.is_empty() {
            eprintln!("⚠️  Warning: Files specified with --generate will be ignored");
        }
        workflow.generate_message_only().await?;
    } else if args.stage_and_generate {
        workflow.stage_and_generate(args.files).await?;
    } else {
        workflow.auto_commit_and_push(args.files).await?;
    }

    Ok(())
}

use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

/// Ralph Wiggum Loop RS — autonomous agent loop that reads prd.json,
/// invokes Claude Code iteratively, detects completion, and tracks progress.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Maximum number of iterations to run
    #[arg(default_value_t = 10)]
    max_iterations: u32,

    /// Path to the agent prompt file
    #[arg(long, default_value = "CLAUDE.md")]
    prompt: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Prd {
    branch_name: String,
    user_stories: Vec<UserStory>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UserStory {
    id: String,
    title: String,
    passes: bool,
}

fn load_prd() -> Result<Prd> {
    let path = PathBuf::from("prd.json");
    let contents = std::fs::read_to_string(&path)
        .with_context(|| "Failed to read prd.json — does the file exist in the current directory?")?;
    let prd: Prd = serde_json::from_str(&contents)
        .with_context(|| "Failed to parse prd.json — is the JSON valid?")?;
    Ok(prd)
}

fn init_progress_file() -> Result<()> {
    let path = PathBuf::from("progress.txt");
    if !path.exists() {
        let now = Local::now();
        let header = format!("# Ralph Progress Log\nStarted: {now}\n---\n");
        std::fs::write(&path, header).with_context(|| "Failed to create progress.txt")?;
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let prd = match load_prd() {
        Ok(prd) => prd,
        Err(err) => {
            eprintln!("Error: {err:#}");
            std::process::exit(1);
        }
    };

    if let Err(err) = init_progress_file() {
        eprintln!("Error: {err:#}");
        std::process::exit(1);
    }

    println!(
        "max_iterations: {}, prompt: {}, branch: {}, stories: {}",
        cli.max_iterations,
        cli.prompt.display(),
        prd.branch_name,
        prd.user_stories.len()
    );
}

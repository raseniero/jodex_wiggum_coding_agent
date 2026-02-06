use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use serde::Deserialize;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

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
#[allow(dead_code)]
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

/// Run a single iteration: read the prompt file, pipe it to claude, stream output.
/// Returns the accumulated stdout as a String.
fn run_iteration(prompt_path: &PathBuf) -> Result<String> {
    let prompt_contents = std::fs::read_to_string(prompt_path)
        .with_context(|| format!("Failed to read prompt file '{}'", prompt_path.display()))?;

    let mut child = Command::new("claude")
        .args(["--dangerously-skip-permissions", "--print"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| "Failed to spawn 'claude' — is it installed and on your PATH?")?;

    // Write prompt to stdin, then drop to close it
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(prompt_contents.as_bytes())
            .with_context(|| "Failed to write prompt to claude stdin")?;
    }

    // Stream stdout line-by-line (tee behavior) while accumulating
    let mut accumulated = String::new();
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line.with_context(|| "Failed to read line from claude stdout")?;
            println!("{line}");
            accumulated.push_str(&line);
            accumulated.push('\n');
        }
    }

    child
        .wait()
        .with_context(|| "Failed to wait for claude process")?;

    Ok(accumulated)
}

fn main() {
    let cli = Cli::parse();

    let _prd = match load_prd() {
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

    for i in 1..=cli.max_iterations {
        println!("===============");
        println!("  Ralph Iteration {i} of {} (claude)", cli.max_iterations);
        println!("===============");

        match run_iteration(&cli.prompt) {
            Ok(_output) => {
                // Completion detection will be added in US-006
            }
            Err(err) => {
                eprintln!("Error: {err:#}");
                std::process::exit(1);
            }
        }

        // Sleep between iterations (skip sleep after last iteration)
        if i < cli.max_iterations {
            thread::sleep(Duration::from_secs(2));
        }
    }
}

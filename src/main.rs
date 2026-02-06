use clap::Parser;
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

fn main() {
    let cli = Cli::parse();
    println!(
        "max_iterations: {}, prompt: {}",
        cli.max_iterations,
        cli.prompt.display()
    );
}

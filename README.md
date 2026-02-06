# Ralph Wiggum Loop RS

A minimal Rust CLI that implements the core Ralph Wiggum autonomous agent loop. Ralph reads a `prd.json` file, invokes Claude Code iteratively, detects a completion signal, and tracks progress in a log file.

## Prerequisites

- **Rust toolchain** — Install via [rustup](https://rustup.rs/) (stable channel)
- **Claude CLI** — The `claude` command must be installed, authenticated, and available on your `PATH`

## Local Development Setup

```bash
git clone <repo-url>
cd agentic_loop_rs
cargo build
```

## Usage

Run with default settings (10 iterations, prompt from `CLAUDE.md`):

```bash
cargo run
```

Run the compiled binary directly:

```bash
ralph
```

Specify a custom number of iterations:

```bash
ralph 20
cargo run -- 20
```

Specify a custom prompt file:

```bash
ralph --prompt path/to/prompt.md
ralph --prompt path/to/prompt.md 15
```

## CLI Arguments

| Argument / Flag    | Description                                      | Default     |
| ------------------ | ------------------------------------------------ | ----------- |
| `max_iterations`   | Optional positional argument. Max loop iterations | `10`        |
| `--prompt <PATH>`  | Path to the agent prompt file                     | `CLAUDE.md` |
| `--help`           | Display usage information                         |             |
| `--version`        | Display version from Cargo.toml                   |             |

## Exit Codes

| Code | Meaning                                                        |
| ---- | -------------------------------------------------------------- |
| `0`  | All tasks complete — `<promise>COMPLETE</promise>` was detected |
| `1`  | Error (missing `prd.json`, spawn failure) or max iterations reached without completion |

## File Structure

```
agentic_loop_rs/
├── src/main.rs                  # Main binary — CLI parsing, loop, completion detection
├── tests/cli.rs                 # Integration tests using assert_cmd
├── Cargo.toml                   # Project manifest and dependencies
├── scripts/ralph/
│   ├── prd.json                 # PRD defining user stories for the agent
│   ├── CLAUDE.md                # Agent prompt/instructions for Claude Code
│   ├── progress.txt             # Progress log (created/appended by the agent)
│   └── ralph.sh                 # Original bash script (reference)
└── tasks/
    └── prd-ralph-loop-rs.md     # Human-readable PRD document
```

## How It Works

1. Ralph reads and validates `prd.json` from the current working directory
2. Initializes `progress.txt` if it doesn't already exist
3. Loops up to `max_iterations` times:
   - Reads the prompt file and pipes it to `claude --dangerously-skip-permissions --print` via stdin
   - Streams Claude's output to the terminal in real-time while capturing it
   - Checks for `<promise>COMPLETE</promise>` in the output — exits with code 0 if found
   - Sleeps 2 seconds before the next iteration
4. If all iterations are exhausted without completion, exits with code 1

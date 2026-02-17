# Jodex Wiggum Loop RS

A minimal Rust CLI that implements the core Jodex Wiggum autonomous agent loop. Jodex reads a `prd.json` file, invokes Claude Code iteratively, detects a completion signal, and tracks progress in a log file.

## Prerequisites

- **Rust toolchain** — Install via [rustup](https://rustup.rs/) (stable channel)
- **Claude CLI** — The `claude` command must be installed, authenticated, and available on your `PATH`

## Local Development Setup

```bash
git clone <repo-url>
cd agentic_loop_rs
cargo build
```

## Building a Release Binary

Build an optimized release binary:

```bash
cargo build --release
```

The binary will be at `target/release/jodex`.

### Optimized for Apple Silicon (M1/M2/M3/M4)

To produce a binary fully tuned for your specific Mac chip, add `target-cpu=native`:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

This enables all CPU-specific instructions (NEON SIMD, etc.) available on your hardware, producing the fastest possible binary for your machine.

> **Note:** Binaries built with `target-cpu=native` are tied to your CPU generation. If you need to distribute to other Macs, omit the `RUSTFLAGS` so the binary stays compatible with all `aarch64-apple-darwin` targets.

The release profile in `Cargo.toml` already includes LTO, single codegen unit, symbol stripping, and abort-on-panic for maximum optimization.

## Usage

Run with default settings (10 iterations, prompt from `CLAUDE.md`):

```bash
cargo run
```

Run the compiled binary directly:

```bash
jodex
```

Specify a custom number of iterations:

```bash
jodex 20
cargo run -- 20
```

Specify a custom prompt file:

```bash
jodex --prompt path/to/prompt.md
jodex --prompt path/to/prompt.md 15
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
├── scripts/jodex/
│   ├── prd.json                 # PRD defining user stories for the agent
│   ├── CLAUDE.md                # Agent prompt/instructions for Claude Code
│   ├── progress.txt             # Progress log (created/appended by the agent)
│   └── jodex.sh                 # Original bash script (reference)
└── tasks/
    └── prd-jodex-loop-rs.md     # Human-readable PRD document
```

## How It Works

1. Jodex reads and validates `prd.json` from the current working directory
2. Initializes `progress.txt` if it doesn't already exist
3. Loops up to `max_iterations` times:
   - Reads the prompt file and pipes it to `claude --dangerously-skip-permissions --print` via stdin
   - Streams Claude's output to the terminal in real-time while capturing it
   - Checks for `<promise>COMPLETE</promise>` in the output — exits with code 0 if found
   - Sleeps 2 seconds before the next iteration
4. If all iterations are exhausted without completion, exits with code 1

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Jodex Wiggum Loop RS is a Rust CLI that implements an autonomous agent loop. It reads a `prd.json` file, invokes `claude --dangerously-skip-permissions --print` iteratively, detects the `<promise>COMPLETE</promise>` completion signal, and tracks progress in `progress.txt`.

The binary is named `jodex`. It requires the `claude` CLI to be installed and on PATH.

## Build & Development Commands

```bash
cargo build                        # Debug build
cargo build --release              # Optimized release build
cargo check                        # Type-check without building
cargo clippy                       # Lint
cargo test                         # Run all tests
cargo test -- --nocapture          # Run tests with stdout visible
cargo test missing_prd             # Run a single test by name substring
```

Apple Silicon optimized build:
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Running

```bash
jodex                              # Default: 10 iterations, reads CLAUDE.md as prompt
jodex 20                           # 20 iterations
jodex --prompt path/to/prompt.md   # Custom prompt file
```

Exit code 0 = all tasks complete. Exit code 1 = error or max iterations reached.

## Architecture

Single-file binary at `src/main.rs` (~147 lines). No library crate, no modules.

**Core flow:**
1. `load_prd()` — deserializes `prd.json` (camelCase JSON → snake_case Rust via serde)
2. `init_progress_file()` — creates `progress.txt` with timestamp header if missing
3. Main loop runs up to `max_iterations`:
   - `run_iteration()` — reads prompt file, spawns `claude` subprocess, pipes prompt via stdin, streams stdout line-by-line while accumulating output
   - Checks accumulated output for `<promise>COMPLETE</promise>`
   - 2-second sleep between iterations

**Key patterns:**
- `anyhow::Result` everywhere; main catches errors with `eprintln!` + `exit(1)`
- `#[serde(rename_all = "camelCase")]` on PRD structs to match JSON field names
- `child.stdin.take()` to write then drop stdin (signals EOF to subprocess)
- `Stdio::inherit()` on stderr for real-time pass-through
- CLI parsing via `clap` derive API

## Testing

Integration tests in `tests/cli.rs` use `assert_cmd` + `predicates`. Tests isolate from the project's own `prd.json` by running in a temp directory via `current_dir()`.

## Key Directories

- `scripts/jodex/` — operational directory with its own `prd.json`, `CLAUDE.md`, and `progress.txt` for running Jodex on itself
- `skills/` — agent skill templates (PRD generation, BRD/PRD, TDS)
- `skills/CLAUDE/CLAUDE.md` — advanced agent prompt template with PR creation support
- `ai_docs/ralph_wiggum_loop/` — documentation on the Ralph autonomous loop pattern

## Release Profile

`Cargo.toml` has an aggressive release profile: `opt-level = 3`, `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`.

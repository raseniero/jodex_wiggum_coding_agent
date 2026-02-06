# PRD: Ralph Wiggum Loop - Rust CLI

## Introduction

Rewrite the core Ralph Wiggum autonomous agent loop as a Rust CLI application. Ralph is a long-running loop that repeatedly invokes Claude Code to work through user stories defined in a `prd.json` file. Each iteration feeds the agent its instructions, captures output, checks for a completion signal, and tracks progress. This Rust version is a minimal MVP focused on the core loop with Claude Code as the only backend.

## Goals

- Provide a compiled, single-binary Rust CLI that replaces the bash `ralph.sh` script for the Claude Code workflow
- Read a `prd.json` file to know what work needs to be done
- Run up to N iterations, invoking `claude --dangerously-skip-permissions --print` each time with the agent prompt piped via stdin
- Detect the `<promise>COMPLETE</promise>` signal in agent output and exit early on success
- Initialize and append to a `progress.txt` log file
- Exit with appropriate status codes (0 = all tasks complete, 1 = max iterations reached)

## User Stories

### US-001: Initialize Cargo project
**Description:** As a developer, I want a properly structured Rust project so that I can build and extend the Ralph loop.

**Acceptance Criteria:**
- [ ] `cargo init` creates a binary project at the repo root with `Cargo.toml` and `src/main.rs`
- [ ] Project name is `ralph` in Cargo.toml
- [ ] `cargo build` succeeds with no errors
- [ ] `cargo clippy` passes with no warnings

### US-002: Parse CLI arguments
**Description:** As a user, I want to run `ralph [max_iterations]` with sensible defaults so that I can control how many iterations to run.

**Acceptance Criteria:**
- [ ] Uses the `clap` crate with derive API for argument parsing
- [ ] Accepts an optional positional argument `max_iterations` (u32, default: 10)
- [ ] Accepts an optional `--prompt` flag to specify the path to the agent prompt file (default: `CLAUDE.md` in the same directory as the binary, falling back to current directory)
- [ ] `ralph --help` displays usage information
- [ ] `ralph --version` displays version from Cargo.toml
- [ ] Invalid arguments produce a clear error message
- [ ] `cargo clippy` passes with no warnings

### US-003: Read prd.json file
**Description:** As the Ralph loop, I need to verify that a `prd.json` file exists before starting so that the agent has work to do.

**Acceptance Criteria:**
- [ ] On startup, checks for `prd.json` in the current working directory
- [ ] If `prd.json` is missing, prints a clear error message and exits with code 1
- [ ] If `prd.json` exists, reads and deserializes it using `serde_json` into a struct
- [ ] The `Prd` struct captures at minimum: `branchName` (String) and `stories` (Vec of story objects with at least `id`, `title`, and `passes` fields)
- [ ] Malformed JSON produces a clear error message and exits with code 1
- [ ] `cargo clippy` passes with no warnings

### US-004: Initialize progress.txt
**Description:** As the Ralph loop, I need to create or verify a `progress.txt` file so that iteration results are logged.

**Acceptance Criteria:**
- [ ] If `progress.txt` does not exist in the current directory, creates it with a header: `# Ralph Progress Log\nStarted: <datetime>\n---\n`
- [ ] If `progress.txt` already exists, leaves it untouched
- [ ] Uses the `chrono` crate (or `std::time`) for datetime formatting
- [ ] `cargo clippy` passes with no warnings

### US-005: Execute the core iteration loop
**Description:** As the Ralph loop, I want to run Claude Code up to N times, piping the prompt file via stdin and capturing output, so that stories get implemented autonomously.

**Acceptance Criteria:**
- [ ] Loops from 1 to `max_iterations`
- [ ] Each iteration prints a banner: `===============\n  Ralph Iteration {i} of {max} (claude)\n===============`
- [ ] Reads the prompt file (from `--prompt` path) and pipes its contents to stdin of `claude --dangerously-skip-permissions --print`
- [ ] Captures stdout and stderr from the claude process
- [ ] Streams output to the terminal in real-time (tee behavior) while also capturing it for completion detection
- [ ] If the process fails to spawn (e.g., `claude` not found), prints a clear error and exits with code 1
- [ ] After each iteration, sleeps for 2 seconds before the next
- [ ] `cargo clippy` passes with no warnings

### US-006: Detect completion signal
**Description:** As the Ralph loop, I want to detect `<promise>COMPLETE</promise>` in the agent's output so that I stop early when all stories are done.

**Acceptance Criteria:**
- [ ] After each iteration, checks the captured output for the string `<promise>COMPLETE</promise>`
- [ ] If found, prints "Ralph completed all tasks!" and "Completed at iteration {i} of {max}"
- [ ] Exits with code 0 on completion
- [ ] If not found, prints "Iteration {i} complete. Continuing..." and proceeds to the next iteration
- [ ] If all iterations exhaust without completion, prints "Ralph reached max iterations ({max}) without completing all tasks." and exits with code 1
- [ ] `cargo clippy` passes with no warnings

### US-007: Integration test with mock
**Description:** As a developer, I want a basic integration test so that I can verify the loop logic works end-to-end.

**Acceptance Criteria:**
- [ ] Test verifies that running `ralph` without a `prd.json` exits with code 1 and prints an error
- [ ] Test verifies CLI `--help` output contains expected program description
- [ ] Tests use `assert_cmd` crate for CLI testing
- [ ] All tests pass with `cargo test`
- [ ] `cargo clippy` passes with no warnings

## Functional Requirements

- FR-1: The binary is named `ralph` and is invoked as `ralph [OPTIONS] [max_iterations]`
- FR-2: Default max iterations is 10 if not specified
- FR-3: On startup, validate that `prd.json` exists and is valid JSON in the current directory
- FR-4: Initialize `progress.txt` with a timestamped header if it does not exist
- FR-5: Each iteration invokes `claude --dangerously-skip-permissions --print` with the prompt file piped to stdin
- FR-6: Agent output is streamed to the terminal in real-time and captured for completion detection
- FR-7: If agent output contains `<promise>COMPLETE</promise>`, exit successfully (code 0)
- FR-8: If max iterations reached without completion, exit with code 1
- FR-9: A 2-second pause occurs between iterations
- FR-10: All errors produce human-readable messages on stderr

## Non-Goals

- No support for the `amp` tool backend (Claude Code only)
- No branch archival or `.last-branch` tracking
- No updating or writing to `prd.json` (read-only)
- No colored/styled terminal output (plain text is fine for MVP)
- No configuration file support
- No daemon/background mode
- No retry logic for failed claude invocations within an iteration

## Technical Considerations

- **Language:** Rust (2021 edition)
- **Dependencies:** `clap` (CLI args), `serde` + `serde_json` (PRD parsing), `chrono` (timestamps), `assert_cmd` (testing)
- **Process execution:** Use `std::process::Command` with piped stdin/stdout. For real-time streaming, read stdout line-by-line while accumulating for completion detection.
- **Error handling:** Use `anyhow` for ergonomic error propagation with context
- **Minimum Rust version:** stable (latest)

## Success Metrics

- `cargo build --release` produces a single static binary
- Binary can successfully orchestrate Claude Code through multiple iterations
- Completion signal detection works reliably
- Clear error messages for all failure modes (missing prd.json, missing claude binary, bad JSON)
- All `cargo clippy` and `cargo test` checks pass

## Open Questions

- Should the progress file be appended to by the Rust binary itself (logging iteration start/end), or is that purely the agent's responsibility?
- Should there be a `--dry-run` mode that prints what would happen without invoking claude?

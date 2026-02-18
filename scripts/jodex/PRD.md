# PRD: Skill Deployment Script

## Overview

**Feature ID:** 001
**Feature Name:** Automated Skill Deployment Script
**Business Objective:** Reduce manual effort and errors when deploying skill definitions from the project source to the user's Claude configuration directory
**Success Metric:** Script deploys all target files (`.md`, `.doc`, `.docx`) with 0 exit code, 0 missing files, and completes in <5 seconds

## Problem Statement

Deploying skill definitions currently requires manually copying files from `./skills/jodex-*` subdirectories to `~/.claude/skills/`, preserving directory structure. This is error-prone (missed files, wrong paths) and tedious when done repeatedly during development.

**Evidence:**
- 5 skill folders (`jodex-gen-brd-prd`, `jodex-gen-prd`, `jodex-gen-prd-task`, `jodex-gen-tds`, `jodex-gen-prd-lite`) with 19+ markdown and resource files across nested subdirectories
- Manual copying requires creating destination directories first
- No automated way to ensure all files are current after edits

## Goals

- Automate deployment of all skill files (`.md`, `.doc`, `.docx`) from `./skills/jodex-*` to `~/.claude/skills/` in a single command
- Preserve source directory hierarchy in the destination
- Provide clear logging of every file deployed
- Ensure idempotent execution (safe to run repeatedly)

## User Stories

### US-001: Deploy all skill files
**As a** skill developer
**I want** a single command that copies all skill files (`.md`, `.doc`, `.docx`) from `./skills/jodex-*` to `~/.claude/skills/`
**So that** I can deploy updated skills and their resource files without manual file management

**Acceptance Criteria:**
- [ ] Script copies every `.md`, `.doc`, and `.docx` file found under `./skills/jodex-*/` (including nested subdirectories like `ai_docs/`, `prd_examples/`, `example_tech_spec_md/`)
- [ ] Files are deployed to `~/.claude/skills/` preserving relative path from `./skills/`
- [ ] Example: `./skills/jodex-gen-brd-prd/SKILL.md` → `~/.claude/skills/jodex-gen-brd-prd/SKILL.md`
- [ ] Example: `./skills/jodex-gen-tds/ai_docs/General AI Behaviour.md` → `~/.claude/skills/jodex-gen-tds/ai_docs/General AI Behaviour.md`
- [ ] Script handles filenames with spaces correctly
- [ ] Lint passes (shellcheck)
- [ ] Unit tests pass

**Validates:** Goal to automate deployment of all skill files

### US-002: Automatic directory creation
**As a** skill developer
**I want** the script to create any missing destination directories automatically
**So that** I don't need to manually set up the directory structure

**Acceptance Criteria:**
- [ ] Script creates `~/.claude/skills/` if it does not exist
- [ ] Script creates all necessary subdirectories (e.g., `~/.claude/skills/jodex-gen-brd-prd/ai_docs/`)
- [ ] No errors when directories already exist
- [ ] Lint passes (shellcheck)
- [ ] Unit tests pass

**Validates:** Goal for zero manual directory management

### US-003: Overwrite existing files
**As a** skill developer
**I want** the script to overwrite existing files in the destination
**So that** I always have the latest version deployed

**Acceptance Criteria:**
- [ ] Running the script overwrites previously deployed files with current source versions
- [ ] Running the script 2+ times in succession produces identical results (idempotent)
- [ ] No confirmation prompts — overwrite is silent and automatic
- [ ] Lint passes (shellcheck)
- [ ] Unit tests pass

**Validates:** Goal for idempotent execution

### US-004: Deployment logging
**As a** skill developer
**I want** the script to output a summary of actions taken
**So that** I can verify what was deployed and catch any issues

**Acceptance Criteria:**
- [ ] Each deployed file logged to stdout: `Deployed: skills/jodex-gen-brd-prd/SKILL.md -> ~/.claude/skills/jodex-gen-brd-prd/SKILL.md`
- [ ] Total file count printed at end: `Deployed X files.`
- [ ] Errors printed to stderr with non-zero exit code
- [ ] Exit code 0 when all files deployed successfully
- [ ] Exit code 1 when source directory `./skills/` does not exist
- [ ] Exit code 1 when no `jodex-*` skill folders found
- [ ] Lint passes (shellcheck)
- [ ] Unit tests pass

**Validates:** Goal for clear logging and error reporting

## Functional Requirements

- FR-1: Script must be a valid bash script named `deploy_skills.sh` at the project root *(Supports US-001)*
- FR-2: Script must target only `./skills/jodex-*/` subdirectories — not other folders under `./skills/` *(Supports US-001)*
- FR-3: Script must recursively find all `*.md`, `*.doc`, and `*.docx` files within matched skill directories *(Supports US-001)*
- FR-4: Script must create destination directories using `mkdir -p` before copying *(Supports US-002)*
- FR-5: Script must use `cp` with overwrite behavior (no `-i` interactive flag) *(Supports US-003)*
- FR-6: Script must print one log line per file deployed to stdout *(Supports US-004)*
- FR-7: Script must print a summary count of files deployed *(Supports US-004)*
- FR-8: Script must exit with code 1 and print error to stderr if source directory is missing or no skill folders match *(Supports US-004)*

## Non-Functional Requirements

- **Portability:** Script must run on macOS (zsh/bash) and Linux (bash). Use POSIX-compatible constructs where possible.
- **Performance:** Script completes in <5 seconds for the current set of 19 files.
- **Safety:** Script must not delete any files — only create directories and copy files.
- **Code Quality:** Script passes `shellcheck` with no warnings.

## Non-Goals

- No backup of existing destination files before overwrite
- No dry-run mode
- No verbose/quiet flags
- No rollback capability
- No deployment of skill folders outside the `jodex-*` pattern
- No `--clean` flag to remove stale destination files
- No CI/CD integration

## Success Metrics

**Primary Metrics:**
- Script exits with code 0 on successful deployment
- File count in destination matches file count in source (`find` comparison yields 0 diff)
- All deployed file contents in destination are byte-identical to source (`diff -r` yields no differences)

**Measurement:**
- Automated test: run script, then compare source and destination with `diff -r`
- Manual verification: run script and review stdout log

## Open Questions

None — all questions resolved.

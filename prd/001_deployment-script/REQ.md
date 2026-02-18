# Requirement Brief: Skill Deployment Script

## Overview
Develop a bash script to automate the deployment of skill definitions from the project's source directory to the user's Claude skills configuration directory.

## Source & Destination
*   **Source:** `./skills/` (Relative to project root) containing:
    *   `jodex-gen-brd-prd`
    *   `jodex-gen-prd`
    *   `jodex-gen-prd-task`
    *   `jodex-gen-tds`
    *   `jodex-gen-prd-lite`
*   **Destination:** `~/.claude/skills/` (User Home Directory)

## Core Requirements
1.  **File Selection:**
    *   Target all markdown files (`*.md`) located within the `./skills/jodex-*` subdirectories.
    *   (Optional but recommended) Consider copying associated resource files if skills depend on them, though the primary requirement specifies `.md` files.

2.  **Directory Structure:**
    *   Preserve the directory hierarchy from the source.
    *   Example: A file at `./skills/jodex-gen-brd-prd/SKILL.md` must be deployed to `~/.claude/skills/jodex-gen-brd-prd/SKILL.md`.
    *   Create any missing subdirectories in the destination path automatically.

3.  **Deployment Behavior:**
    *   **Overwrite:** The script should overwrite existing files in the destination to ensure the latest version is deployed.
    *   **Idempotency:** Running the script multiple times should have the same result without errors.

4.  **Usability & Logging:**
    *   Output a summary of actions taken (e.g., "Deployed: skills/brd-prd/SKILL.md -> ~/.claude/skills/brd-prd/SKILL.md").
    *   Report any errors encountered (permissions, missing source, etc.).

## Acceptance Criteria
*   The script is a valid bash script (e.g., `deploy_skills.sh`).
*   Execution copies all `.md` files from `./skills/` to `~/.claude/skills/` maintaining structure.
*   No manual directory creation is required by the user; the script handles it.

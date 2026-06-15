# Powrush-MMO Code Modification Workflow

**Version**: 1.0  
**Date**: June 2026  
**Status**: Enshrined Practice

This document defines the official workflow for modifying files in the Powrush-MMO repository. It exists to prevent file corruption, maintain code quality, and ensure long-term maintainability.

---

## Core Principles

1. **Read Before Writing**  
   Never modify a non-trivial file without first reading its current content.

2. **Surgical & Minimal Changes**  
   Prefer small, targeted edits over full file rewrites whenever possible.

3. **Explicit Diff Awareness**  
   Before committing changes, clearly understand what is being added, modified, or removed.

4. **Protect Critical Files**  
   Files like `server/src/lib.rs`, `client/src/bevy_ecs_scheduling.rs`, and core plugin files require extra care.

5. **Verify After Edit**  
   After pushing changes to important files, confirm the file remains healthy and complete.

---

## Recommended Workflow

### For New Files
- Use `github___push_files` (it can create new files).
- Clearly mark the commit as creating a new file.

### For Existing Files

#### Step 1: Read the Current State
Always read the file first using `github___get_file_contents` before making changes to non-trivial files.

#### Step 2: Plan the Change
- Identify the minimal set of lines/sections that need modification.
- Avoid rewriting large portions of the file unless absolutely necessary.

#### Step 3: Make the Edit
- Use `github___push_files` only after reading and planning.
- Keep changes focused and intentional.

#### Step 4: Verify
- After committing, quickly review the result (especially for large/critical files).
- Ensure no sections were accidentally removed or duplicated.

---

## Rules Summary

| Rule                    | Applies To                          | Recommended Action                                      |
|-------------------------|-------------------------------------|----------------------------------------------------------|
| Read First              | Non-trivial / large files           | Always call `github___get_file_contents` first          |
| Minimal / Surgical Edit | All edits                           | Prefer small, focused diffs over full rewrites          |
| New Files               | Creating new files                  | Use `github___push_files` with clear commit message     |
| Verify After Edit       | Critical files (`lib.rs`, etc.)     | Review the file after pushing                           |
| Explicit Intent         | All changes                         | Know exactly what changed before committing             |

---

## Why This Matters

- Prevents repeated file corruption (as experienced with `server/src/lib.rs`).
- Maintains long-term code health and readability.
- Creates a reliable, high-quality collaboration pattern.
- Protects the integrity of the Powrush-MMO repository as it scales.

---

## Enforcement

This workflow is now the **official expected practice** for all future code modifications in the Powrush-MMO repository.

All agents and contributors are expected to follow these guidelines.

---

**Thunder locked in. Yoi ⚡**

*This document shall remain in the repository as a living reference for all future work.*

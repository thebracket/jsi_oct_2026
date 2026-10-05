---
name: update-readme
description: Keep the repository README in sync with the example crates and mdBook chapters. Use when an example is added/renamed, a manual page is added/renamed, SUMMARY.md changes, or the user asks to update the README.
---

# Update README

The repository `README.md` is deliberately minimal. It contains only:

1. Title and any project banner
2. `## Repository Layout`
3. `## Getting the Code`
4. `## Viewing the Manual`
5. `## Examples` (table)
6. `## Contents` (mirrors `SUMMARY.md`, with placeholders marked)

Do **not** add extra preamble or explanation beyond the project's established
conventions. Keep it to the sections above.

## When to use

- A new example crate is added under `code/`, or an existing one is renamed.
- A manual page is added, renamed, or removed under `manual/src/`.
- `manual/src/SUMMARY.md` changes.
- The user asks to "update the README" or "keep the README in sync".

## Sources of truth

- `code/Cargo.toml` → `[workspace] members` is the list of examples.
- `manual/src/SUMMARY.md` → the book structure. Empty `()` links are
  placeholders (chapters that do not exist yet).
- `manual/src/**/*.md` → which chapters actually exist on disk.
- `AGENTS.md` → repo background, if present.

## Workflow

1. Read `README.md`, `code/Cargo.toml`, and `manual/src/SUMMARY.md`. List
   `code/ex*/` and the files under `manual/src/`.
2. Update the **Examples** table. One row per workspace member, in member order.
   Link the crate directory (`code/<example>`) and its manual chapter, or `—`
   when there is no chapter yet. Reuse the existing "What it shows" wording for
   unchanged rows; write a short one-line description for new ones.
3. Update **Contents** to mirror `SUMMARY.md` exactly:
   - chapter file exists → `[Title](manual/src/path.md)`
   - chapter file missing → plain text `Title (placeholder)`
   Match the nested indentation of `SUMMARY.md`.
4. Update **Repository Layout** only if a directory was added or renamed.
5. Verify that every relative link in the README resolves. Fix anything broken.

## Conventions

- Relative links are written from the repository root, e.g.
  `manual/src/intro/intro.md` and `code/ex01_movefail`.
- Placeholders are plain text with a trailing ` (placeholder)` — never a broken
  link, and never an empty link.
- Mirror `SUMMARY.md` spelling exactly, including apparent typos. Do not silently
  correct them; ask first.
- Keep the prose terse. Tables and lists, not paragraphs.

## Secrets

Never stage or commit `.env` files, `code/target/`, or `manual/book/`. They are
gitignored; verify with `git diff --cached --name-only` before committing, and
scan the diff for key-like values if in doubt.

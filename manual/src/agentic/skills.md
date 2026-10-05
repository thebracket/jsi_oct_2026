# Skills

Agents default to generic code. Left to their own devices, they reach for a `String` status field, a bare `as` cast, and a plain `u64` for every identifier---the lowest common denominator of every language they've ever read. A *skill* is how you fix that: a folder containing a `SKILL.md` file that tells the agent how you want things done. OpenCode, Claude Code, and Codex all auto-discover them, and the `description:` in the frontmatter is what the agent matches against the task at hand to decide whether to load it---so write the description like a trigger, not a title:

```yaml
---
name: rust-design
description: Rust design rules for invariants, newtypes, enums over strings, safe
  conversions, and module-local errors. Use when designing types, function
  contracts, or error handling.
---
```

The philosophy: encode your *taste* once, and every agent session inherits it. This repository carries four downloadable examples under `skills/`---copy a folder into your agent's skills directory (`.opencode/skills/` or `.claude/skills/`) and it gets picked up in the next session.

## What I Was Aiming For

The flagship is `skills/rust-design/SKILL.md`, and its rules are the ones I most want agents to internalize:

* **Make invalid state unrepresentable.** Push business rules into the type system so the compiler---not the reviewer---enforces them.
* **Newtypes over bare primitives.** Two different IDs that can be accidentally mixed are a bug waiting to happen; units of measure belong in the type, too.
* **Enums, not strings, for status.** A common antipattern: a `String` status field accepts `"redy"` just as happily as `"ready"`. An enum turns typos into compile errors and makes every `match` exhaustive.
* **Safe conversion.** `From`/`TryFrom`/`try_into()` over bare `as` casts; checked arithmetic unless overflow is genuinely impossible.
* Plus supporting habits: `let else` and early returns over deep nesting, module-local errors via `thiserror`, small pure helpers, and never holding a lock across I/O or `.await`.

The other three skills cover the rest of the agent workflow:

| Skill | What it's for |
|---|---|
| `skills/rust-design/SKILL.md` | The Rust idiom rules above---design pressure applied to new and touched code, not a refactoring mandate. |
| `skills/anti-slop/SKILL.md` | The "smallest-solution ladder" plus a code-slop checklist: no `todo!()`, no `println!` where `tracing` belongs, no dead code, generic names, or magic numbers. |
| `skills/quality-gates/SKILL.md` | What "done" means: `cargo fmt -- --check`, warning-clean scope, clippy with `-D warnings` on boundary changes, and dependency review with `cargo audit` and `cargo machete`. |
| `skills/update-readme/SKILL.md` | A simple project-chore skill: keep the README's examples table and contents mirror in sync with `code/Cargo.toml` and `manual/src/SUMMARY.md`. |

> These are generalized from skills I use daily on the libreplanner project---the originals are the same rules with more opinions attached. Start small: one skill with a well-crafted description beats a pile of skills nobody triggers.

## Writing Your Own

When you catch yourself giving the same correction twice, that's a skill waiting to be written. Draft it, hand it to your agent along with a few example files, and ask it to extract the rules. Then---like everything else in agent-led development---review it yourself: the skill will shape every future session, so an incorrect rule costs you every time it fires.

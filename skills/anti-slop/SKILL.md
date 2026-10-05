---
name: anti-slop
description: Zero-slop workflow. Use when writing or reviewing code, docs, or user-facing text. Drive slop toward zero with the smallest-solution ladder and a code-slop checklist.
---

# Anti-Slop Workflow

Use this skill when writing or reviewing code, docs, or any user-facing text in this repo.

The goal is **zero slop**.

## Smallest-Solution Ladder

Read the code the change touches and trace the real flow first. Then, before writing anything,
stop at the first rung that holds:

1. Does this need to exist? If not, skip it.
2. Does the repo already do it? Reuse the existing helper, type, or service.
3. Does the standard library do it? Use it.
4. Does the platform or framework already provide it? Use that before adding a dependency.
5. Does an installed dependency do it? Use it instead of adding another.
6. Is the change one line? Keep it one line.
7. Otherwise, write the minimum that works, and nothing more.

The ladder shrinks the solution, never the requirements. It never licenses cutting
trust-boundary validation, error handling, data-loss handling, or security; when it appears to
conflict with an explicit rule in `AGENTS.md` or your project skills, those rules win. Do not
deliberate over rungs that clearly do not apply: the point is to avoid over-building, not to
add a review round.

## Review Pass

1. Make your change.
2. Before returning to the user, critically review the diff whenever the repo changed.
3. If the review finds slop, fix it and re-review until it is clean---or explicitly explain why a residual issue is intentionally left in scope.

Duplication, dead code, and overcomplication findings are part of the review itself.

## Code Slop

High-confidence slop:

- `todo!()`, `unimplemented!()`, placeholder stubs, or commented-out code
- raw `println!` or `eprintln!` in Rust product code instead of `tracing`
- noisy or inflated logging levels, such as logging normal lifecycle events above `debug!` or bad credentials as warnings/errors
- dead code, unused helpers, or warning suppressions used to hide avoidable issues
- duplicated helpers or copy-paste logic that should reuse an existing helper or module
- generic names that hide meaning
- abstractions with no operational payoff
- crate extraction that merely relocates a monolith without creating focused modules
- required trait behavior hidden behind a default implementation that only returns an unavailable error
- per-request construction of low-level stores or duplicate process-wide indexes and caches
- unbounded strong-reference caches for keyed services
- lazy algorithms such as avoidable repeated scans, parsing, cloning, or allocation
- magic numbers with no named constant or explanation
- crate roots that turn into mixed implementation dumps instead of thin module maps
- multi-thousand-line source or test files that should have been split into focused sibling modules
- new `std::sync::Mutex` or `std::sync::RwLock` usage where `parking_lot` would work without loss of correctness
- unit tests moved away from the file they directly exercise without a concrete reason
- integration tests mixed into implementation files instead of separate test modules
- test-only helpers or modules that are not protected by `#[cfg(test)]`

Cleanup direction:

- Delete dead code instead of silencing warnings.
- Prefer one clean shared helper over near-duplicate variants.
- Keep modules focused and function names intent-revealing.
- Keep `lib.rs` and `main.rs` small when feasible: module declarations, crate docs, exports, and little else.
- When a crate root starts interleaving public types, internal state, and tests, split it into focused `filename.rs` modules.
- When a file grows into the high hundreds or beyond, look for a real module boundary before it turns into a multi-thousand-line dump.
- Prefer `parking_lot` for synchronous lock usage unless a specific standard-library constraint forces `std::sync`.
- Keep true unit tests beside the code they test, and move broader integration coverage into separate test modules.
- Keep test-only items out of normal builds with `#[cfg(test)]`.
- Validate invariants early and keep error handling explicit.

## Text Slop

Avoid filler and AI-ish phrasing in docs, comments, and user-visible strings:

- no stage directions such as "In this section we will..."
- no filler transitions such as "Let's take a closer look"
- no buzzword sludge
- minimize hedging when certainty is available
- prefer direct active statements

## Notes

- Existing rough edges elsewhere in the repo are not permission for unrelated cleanup.

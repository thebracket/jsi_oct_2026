---
name: quality-gates
description: Rust quality gates for formatting, warning-clean verification scope, clippy, architecture boundaries, dependency review, and release hygiene. Use when verifying Rust changes before considering them complete.
---

# Rust Quality Gates

Use this skill when verifying Rust changes before considering them complete.

## Required Verification

- Start with your project's baseline validation flow (format, compile, test), then add `cargo fmt -- --check` before considering the work complete.
- Keep the touched Rust scope warning-clean under the chosen validation commands.
- Treat the final unfiltered scoped tests and strict Clippy run as one completion gate per coherent implementation batch. Use focused checks and filtered behavior tests during iteration instead of repeating that gate after every small repair.
- For a cross-crate boundary change, run `cargo check --workspace --all-targets` and `cargo clippy --workspace --all-targets -- -D warnings` after the focused crate gates. All-target compilation catches missing trait methods and test-only boundary drift that a product-only build can miss.
- Re-run architecture-boundary checks against the final tree: subsystem-specific logic must remain in its owning subsystem. Exclude test fixtures deliberately rather than accepting unexplained matches.
- Treat a full-workspace test failure separately from affected-scope proof. Investigate enough to classify it, report the exact failing or stalled test, and never use an unrelated baseline failure to waive focused behavioral coverage.

## Preferred Verification Commands

Use the smallest command that validates the touched area correctly:

- `cargo fmt -- --check`
- `cargo clippy -p <crate> -- -D warnings`
- `cargo test -p <crate>`

When the change is non-trivial or the touched scope is performance-sensitive, also consider:

- `cargo clippy -p <crate> -- -W clippy::pedantic`

Pedantic Clippy is encouraged, but do not treat existing unrelated pedantic findings elsewhere in the repo as permission for broad cleanup.

## Dependency Review

When dependencies change, before release, or when chasing dependency bloat:

- run `cargo audit` when it is available
- run `cargo machete` when it is available
- inspect `cargo tree` to understand large or surprising dependency additions

Interpret results carefully:

- do not ignore findings by default
- document why a finding is acceptable if it is provably non-shipping or otherwise not relevant to the deployed surface
- prefer maintained crates and avoid unnecessary new dependencies

## Notes

- These gates complement, not replace, your project's code-review expectations.
- Use judgment on the smallest correct verification scope, but do not skip verification just because the diff is small.

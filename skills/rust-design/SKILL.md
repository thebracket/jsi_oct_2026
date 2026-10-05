---
name: rust-design
description: Rust design rules for invariants, newtypes, enums over strings, safe conversions, checked arithmetic, module-local errors, and resource ownership. Use when designing Rust types, function contracts, business logic, or error handling.
---

# Rust Design

Use this skill when designing Rust types, function contracts, business logic, or module-local error handling.

## Scope

- Applies to Rust domain models, helper functions, parsing, validation, internal protocols, and module-local implementation choices.
- These rules are design pressure, not a mandate for large refactors: apply them to new and touched code without doing unrelated cleanup.

## Error Handling Rules

- Prefer `thiserror` for new structured errors close to the failure source.
- Scope error types to the module or subsystem that emits them instead of building one project-wide catch-all error.
- Error variants should describe concrete failure modes, not just wrap an inner error with no extra meaning.
- Avoid leaking internal implementation details in user-facing errors. Log operational detail separately when needed.
- Check invariants early and return explicit errors instead of relying on downstream failures.

## Type And Invariant Rules

- Look for ways to make invalid state unrepresentable.
- Prefer enums over free-form strings for statuses, modes, and state transitions.
- Prefer newtypes for identifiers when mixing two IDs would be a logical bug.
- Prefer newtypes for units of measure when the unit matters to correctness.
- Keep compatibility boundaries explicit when serialized formats, external files, or protocol surfaces are involved.
- Give each persisted or cached namespace one identifier domain. Resolve identifiers at ingress and use the same typed key for reads, writes, invalidation, and cleanup.

## Conversion And Arithmetic Rules

- Avoid bare `as` conversions unless there is a concrete reason and the choice is documented at the call site.
- Prefer `From`, `TryFrom`, `into()`, and `try_into()` for type conversion.
- Use checked arithmetic unless overflow is logically impossible or an explicit wrapping or saturating behavior is the intended contract.
- If wrapping or saturating behavior is required, use the explicit operation rather than relying on implicit overflow assumptions.

## Function And Implementation Rules

- Validate preconditions at the top of the function.
- Prefer `let else` and early returns over deep nesting.
- Prefer pure functions when the logic does not need I/O, mutation, or shared state.
- Keep mutable sections as small as possible.
- Avoid repeated cloning, parsing, allocation, or scanning in likely hot paths.
- Use descriptive names that reveal intent instead of generic names such as `data`, `value`, or `result`.
- Prefer small direct helpers over abstract layers that do not pay for themselves.

## Service And Cache Ownership

- Give each low-level resource one construction owner. Pass long-lived services to consumers with `Arc` or a focused interface instead of reconstructing them per request.
- Do not define a shadow consumer trait when the owning subsystem can expose the neutral contract directly.
- Make behavior required by every consumer a required trait method. Do not hide a missing implementation behind a default method that returns an unavailable error at runtime.
- Never hold a synchronous cache lock across I/O, allocation-heavy construction, or `.await`. Construct outside the lock, then double-check before insertion when races are harmless.
- Use `Weak` entries for keyed service caches when callers own the useful lifetime; prune expired entries so the cache does not retain every identifier ever seen.
- Keep dynamic dispatch and allocation outside measured hot loops unless evidence justifies the cost.

# Taming Compile Times

Rust is infamous for compilation times getting painful. It's improving rapidly, but there's still a lot you can do to help yourself. When you're working with coding agents this matters *doubly*---agents live on a tight loop of edit → compile → test, so every second of build time is multiplied by a tireless collaborator.

The core guidance:

* Get fast storage. Storage speed makes a huge difference!
* Use workspaces to share build artifacts. You don't need to rebuild `serde` every time you test part of your workspace!
* Break code into modules---in separate files/directories. Rust can compile modules concurrently, while a single large file has to be processed together.
* Divide into crates. Crates won't be recompiled at all if they didn't change.
* Use macros sparingly.
* Go easy on *generics*. The compiler generates a separate copy of the code for every type you use with a generic function or struct---so "generics with many types" means "the compiler writes you a lot of code". Limit the types you throw at a generic, or use a trait object (`dyn`) where you can afford dynamic dispatch. It also helps to avoid exposing generics across crate boundaries when you can---provide concrete functions/types instead, and the cost stays inside one crate.

## Strip Debug Symbols from Dependencies

You rarely step *into* a dependency in a debugger---you step into *your* code. Debug info for every dependency bloats `target` and slows linking. Turn it off for dependencies while keeping it for your own crates:

```toml
# Cargo.toml
[profile.dev.package."*"]
debug = false
```

If you do need to debug into a dependency, flip it back on temporarily.

## Normalize Your Dependencies

Duplication is a silent compile-time killer: if two of your dependencies need different versions of the same crate, you compile it twice (or more). Find the duplicates:

```bash
cargo tree -d
```

Then normalize: align version requirements across your crates (a `[workspace.dependencies]` section makes one version the single source of truth in a workspace), and update to a common version where you can.

This is a *great* task to hand to a coding agent: *"go through the dependencies and normalize versions to reduce duplication"*. While it's in there, have it run:

```bash
cargo audit
```

`cargo audit` checks your `Cargo.lock` against the RustSec advisory database. Version normalization shrinks your build; the audit makes sure the versions you settle on aren't known-bad. Two birds, one agent.

## Time Your Builds

You can't optimize what you can't see, and neither can your agent:

```bash
cargo build --timings
```

This generates an HTML report (in `target/cargo-timings/`) showing every compilation unit, how long it took, and where units waited on each other. Hand it to your agent---*"read the timings report and tell me what to split up, feature-gate, or drop"*---and it has hard data to work with instead of vibes.

## Cache It with sccache

`sccache` didn't exist when I first wrote this section---it's a compiler cache that survives `cargo clean`, works across projects, and can even share caches between machines:

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
```

With the wrapper set, unchanged dependencies are restored from cache instead of recompiled---turning many "clean build" scenarios into near-no-ops. `sccache --show-stats` tells you how much time it's saving. Agents also love to `cargo clean` at the most inconvenient moments; sccache makes that harmless.

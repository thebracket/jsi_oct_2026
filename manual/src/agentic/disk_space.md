# Disk Space

Rust projects are disk hungry. `target` directories routinely grow into gigabytes, `~/.cargo` keeps every crate you have *ever* downloaded, and coding agents make it worse---they build constantly, happily run in parallel, and love to leave a trail of worktrees behind them. Some guidance:

## Strip Your Binaries

Rust binaries carry debug info and symbols by default, and they're big. Strip them:

```toml
# Cargo.toml
[profile.release]
strip = true
```

Recent Cargo versions already strip debug info from release builds when you haven't asked for debug info---but the symbols remain. `strip = true` removes the lot. Pair it with the dependency debug-info switch from [Compile Times](./compile_times.md) and both your binaries *and* your `target` directory shrink dramatically.

For development builds, there's a middle ground between "full debug info" and "none": `debug = "line-tables-only"` keeps line numbers in your panic backtraces---the part you actually read---while dropping most of the weight:

```toml
[profile.dev]
debug = "line-tables-only"
```

## Share a Target Directory

Every project gets its own `target` directory---and so does every `git worktree`. If you run concurrent agents in worktrees (a great pattern!), each one rebuilds the world into its own private gigabytes. Point them all at one shared target:

```bash
export CARGO_TARGET_DIR=~/.cargo-target
```

or make it permanent in `~/.cargo/config.toml`:

```toml
[build]
target-dir = "/home/you/.cargo-target"
```

Cargo's locking keeps concurrent builds from clobbering each other, and N worktrees now share one cache instead of N copies of the world. Do `git worktree remove` the worktrees themselves when an agent's task merges---the shared target handles the bulk, but abandoned checkouts still nibble.

## Turn Off Incremental (When It Isn't Earning Its Keep)

Incremental compilation caches are one of the two biggest `target` consumers (debug info is the other). `CARGO_INCREMENTAL=0`---or `[profile.dev] incremental = false`---turns them off. Everybody does this in CI, and the agent angle is that *agents* are the new CI: a big AI-driven refactor invalidates the incremental caches wholesale anyway, so you pay the disk cost and still get a rebuild storm. It's fine to leave incremental on while you hand-edit one function; switch it off while the agent churns.

## Clean Regularly

```bash
cargo clean
```

erases the `target` directory---safe to do any time; your sources and `Cargo.lock` are untouched, the next build just recompiles. You can be selective, too: `cargo clean --release` for one profile, or `cargo clean -p mycrate` for one package.

To find the spots you've missed (that workspace you forgot about in October...), `cargo cleaner` scans a parent directory for cargo projects and gives you a TUI for picking which `target` directories to delete. For a less blunt instrument, `cargo sweep` removes stale build artifacts (by age or toolchain) rather than everything.

## Machete the Unused Dependencies

Every dependency you don't use is still downloaded, compiled, cached---and it slows every clean build. `cargo machete` finds them:

```bash
cargo install cargo-machete
cargo machete
```

It's fast *because* it's imprecise: it text-searches for crate names instead of compiling. Macros and build scripts can generate false positives; list those in `[package.metadata.cargo-machete]`. Unused-dependency detection is also on its way into Cargo itself. This pairs perfectly with the "normalize versions" pass from [Compile Times](./compile_times.md)---hand your agent both: *"machete the unused dependencies, then normalize the versions that are left."*

## Manage the Build Cache

`~/.cargo` grows forever: every version of every crate you've ever touched, extracted sources, git checkouts. `cargo cache` shows where it all went:

```bash
cargo install cargo-cache
cargo cache
cargo cache --autoclean
```

`--autoclean` removes the extracted source checkouts (the archives and git repos are kept---Cargo re-extracts them on demand), and `cargo cache --gc` recompresses the git dependency database---a few hundred megabytes back for free in dep-heavy projects.

Watch the other caches, too. If you use sccache from the compile-times page, it has its *own* cache, and it quietly grows forever: `cargo cache sccache` shows its size and hit rate, and `SCCACHE_CACHE_SIZE="20G"` bounds it.

Your toolchains and installed binaries count, too. Every old nightly is about a gigabyte of nostalgia (`rustup toolchain uninstall`), and `~/.cargo/bin` fills with every `cargo install` you've ever tried. `cargo cache toolchain` prints the stats---hand that audit to your agent.

## Going Further

Two aggressive options for the brave:

* **Put the shared target on a tmpfs/ramdisk.** Point `CARGO_TARGET_DIR` at a tmpfs: builds get *faster* (it's RAM), and the whole cache vaporizes on reboot---auto-clean as a physics feature. Only sane with the shared-target setup and enough RAM to bound it.
* **Schedule the hygiene instead of remembering it.** A cron/systemd timer running `cargo-clean-all` or `cargo sweep --time 30` weekly keeps the disk honest without anyone thinking about it. If CI caches `CARGO_HOME`, cargo-cache even ships a `ci-autoclean` build feature that shrinks the cache before upload.

> A good periodic prompt for your agent: *"check disk usage on the cargo target dirs and caches, machete unused dependencies, prune old toolchains and binaries, and tell me what's safe to clean."* Disk hygiene is exactly the kind of chore agents never get bored of.

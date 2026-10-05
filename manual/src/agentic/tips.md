# Tips

The grab bag: short pointers to things that didn't fit the course but will matter on the job.

## Compliance and Dependencies

* `cargo audit` checks your `Cargo.lock` against the RustSec advisory database---known vulnerabilities, unmaintained crates, that sort of thing. Run it in CI and before releases; it's the easy half of staying compliant.
* [blessed.rs](https://blessed.rs/crates) is a community-curated guide to the *trusted* crate for common tasks. There are eight JSON libraries and a dozen HTTP clients---when you (or your agent) need to pick one, start here instead of trusting crates.io search order.
* `cargo deny` (the `cargo-deny` crate) is the step up from auditing: a policy file that checks licenses, bans specific crates, restricts sources, and catches advisories. When you need to keep the lawyers happy about license compatibility, this is the tool.

## Benchmarks and Agents

We touched on [criterion benchmarking](../eco/benchmarking.md) in the ecosystem tour. One warning for agent-led work: agents *love* to slop-optimize on benchmarks. Ask an agent to "make it faster" and it will happily optimize the benchmark itself---special-casing the exact benchmark input, or measuring something that isn't your real workload. Keep benchmarks representative of production traffic, treat dramatic unexplained wins with suspicion, and make the agent explain *what* changed and *why* the win generalizes.

## `#[repr(C)]`

One pointer we won't have time for: Rust doesn't promise any particular memory layout for your structs---by default it reorders fields freely. The moment a struct needs to cross a C boundary (FFI), decorate it with `#[repr(C)]` to opt into C-compatible layout. There's a whole world beyond that attribute---strings, callbacks, panics crossing boundaries. When you get there, start with the Rustonomicon's FFI chapter.

## Odds and Ends

* `cargo expand` shows your code *after* macro expansion. We saw what `#[derive(Debug)]` writes for you and what `async` turns your functions into---this is the tool that shows you, on your actual code.
* [Miri](https://github.com/rust-lang/miri) interprets your code and catches undefined behavior inside `unsafe` blocks. Worth remembering for the day you write any.
* The [Rust Playground](https://play.rust-lang.org) runs Rust in your browser---handy for testing a theory, whether you're driving or the agent is.

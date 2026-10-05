# Don't Hold a Mutex Over an Await!

If you learn only one rule for async Rust, make it this one: **never hold a mutex guard across an `.await`**.

It's very easy to write. We have shared state, so let's protect it with a `Mutex`---and keep the lock while we work:

```rust
use std::sync::{Arc, Mutex};

async fn increment(counter: Arc<Mutex<u32>>) {
    let mut lock = counter.lock().unwrap();
    do_something().await;
    *lock += 1;
}

async fn do_something() {}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let handle = tokio::spawn(increment(counter.clone()));
    handle.await.unwrap();
    println!("Counter: {}", *counter.lock().unwrap());
}
```

The moment you hand the future to the runtime with `tokio::spawn`, the compiler stops you:

```text
error: future cannot be sent between threads safely
   --> src/main.rs:14:31
    |
14  |     let handle = tokio::spawn(increment(counter.clone()));
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ future returned by `increment` is not `Send`
    |
    = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `std::sync::MutexGuard<'_, u32>`
note: future is not `Send` as this value is used across an await
   --> src/main.rs:5:20
    |
4   |     let mut lock = counter.lock().unwrap();
    |         -------- has type `std::sync::MutexGuard<'_, u32>` which is not `Send`
5   |     do_something().await;
    |                    ^^^^^ await occurs here, with `mut lock` maybe used later
note: required by a bound in `tokio::spawn`
   --> .../tokio-1.53.2/src/task/spawn.rs:176:21
    |
176 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
```

`MutexGuard` isn't `Send`---you can't carry a lock between threads. When you `.await`, the runtime is free to move your task to whatever thread has capacity. Holding the guard across the `.await` makes your *entire future* `!Send`, so `tokio::spawn` rejects it.

That's the good outcome: a compile-time bug. The bad outcome is when the code *does* compile---for example, you never spawn it and just `.await` it from `main`. Now you're blocking the executor: while your task sits at the `.await` holding the lock, no other task can touch that mutex---and on a single-threaded runtime, nothing else runs at all. Two tasks holding each other's locks across awaits is a deadlock you can't even see.

The fix is simple: scope the lock so it drops *before* you await:

```rust
async fn increment(counter: Arc<Mutex<u32>>) {
    {
        let mut lock = counter.lock().unwrap();
        *lock += 1;
    } // The guard is dropped here---before the await
    do_something().await;
}
```

Lock, do the work, drop the lock, *then* await. If the work you need under the lock is genuinely async, restructure so the locked part is quick and synchronous.

> What if you really can't avoid holding a lock across an `.await`? `tokio::sync::Mutex` exists for exactly that---its guard is `Send` and designed to be held across await points. It's slower than `std::sync::Mutex`, and reaching for it is usually a design smell. Ask yourself whether the locked work can be done in one synchronous pass. When you truly need it (say, shared state updated via async calls), it's there.

> The code for this is in `code/ex41_mutex_over_await` (the fixed version).

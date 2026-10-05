# Channels

Channels are advertised as Go's superpower, but Rust has them too---and you'll find yourself recommending them a lot. Rob Pike famously said: *"Don't communicate by sharing memory; share memory by communicating."* After the last page, that should sound like good advice: instead of sharing data behind a mutex, a channel lets tasks *send messages* to each other---no locks, and therefore no lock-related foot-guns.

Tokio mirrors the standard library's channels with async versions:

> The code for this is in `code/ex45_channels`.

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    const NUM_TASKS: i32 = 4;
    let (tx, mut rx) = mpsc::channel::<i32>(10); // Tokio channels must have a max size

    for i in 0..NUM_TASKS {
        let my_tx = tx.clone();
        tokio::spawn(async move {
            my_tx.send(i).await.unwrap();
        });
    }

    for _ in 0..NUM_TASKS {
        let received = rx.recv().await.unwrap();
        println!("{received}");
    }
}
```

That's *mpsc*: **multiple producers, single consumer**. Clone the transmitter for every task, send away---and the receiver sees it all. Notice the `10`: Tokio channels are *bounded*, meaning at most that many messages can sit in the queue awaiting processing. That's a feature: if the receiver falls behind, `send` awaits---the channel applies backpressure automatically rather than eating all your memory.

Sending requires `.await`, receiving requires `.await`---channels are fully async, and very efficient on the Tokio side of the fence.

## The Command Pattern (a.k.a. Actors)

You can send any `Send` type over a channel, but I'm particularly fond of sending *enums*---a "command pattern":

```rust
enum Command {
    Increment,
    Decrement,
}

struct Counter {
    value: i32,
}

impl Counter {
    fn execute(&mut self, command: Command) {
        match command {
            Command::Increment => self.value += 1,
            Command::Decrement => self.value -= 1,
        }
    }
}
```

One task owns the `Counter` and sits in a loop executing commands off its channel; everyone else sends commands. This is the *actor model*: by giving a single owner the data, you sidestep locking entirely---no deadlocks, and it scales nicely with multiple producers. Think of it as an in-process microservice. (And notice it's your enum skills paying off again: the compiler enforces that every command is handled.)

## Going Further

Three pointers for when you need more:

* **Bridging sync and async**: a common pattern is to use a `std::sync::mpsc` channel to hand work from the async world to system threads (great for CPU-bound batch processing), and a Tokio channel plus `tokio::runtime::Handle::current()` to send replies *back* into async-land.
* **Broadcast channels**: `tokio::sync::broadcast` allows multiple *receivers*---one message, every subscriber. Great for "shut down now" signals and event fan-out.
* **`select!`**: Tokio's `select!` macro waits on multiple futures at once and acts on whichever completes first---handy for "process messages *or* listen for a quit signal".

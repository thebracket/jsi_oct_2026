use std::sync::{Arc, Mutex};

async fn increment(counter: Arc<Mutex<u32>>) {
    {
        let mut lock = counter.lock().unwrap();
        *lock += 1;
    } // The guard is dropped before we await
    do_something().await;
}

async fn do_something() {
    println!("Doing other work while the lock is free");
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        handles.push(tokio::spawn(increment(counter.clone())));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("Counter: {}", *counter.lock().unwrap());
}

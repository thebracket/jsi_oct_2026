async fn say_hello() {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {
    say_hello().await;
}

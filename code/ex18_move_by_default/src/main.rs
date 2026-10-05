fn do_something(s: String) -> String {
    // Something happens here
    s
}

fn main() {
    let s = "Hello".to_string();
    let s = do_something(s);
    println!("{s}");
}

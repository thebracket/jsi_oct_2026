fn do_something(s: &mut String) {
    // Something happens here
    *s += " World";
}

fn main() {
    let mut s = "Hello".to_string();
    do_something(&mut s);
    println!("{s}");
}

use std::convert::TryInto;

fn main() {
    let i: i64 = 500;
    let i: i32 = i.try_into().unwrap();
    println!("{i}");
}

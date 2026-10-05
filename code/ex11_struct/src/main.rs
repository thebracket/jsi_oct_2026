#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: i32,
}

fn main() {
    let value = MyStruct {
        a: 1,
        b: 2,
    };
    println!("{value:#?}");
}

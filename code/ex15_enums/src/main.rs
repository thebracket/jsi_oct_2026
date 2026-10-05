enum MyEnum { A, B }

impl MyEnum {
    fn new() -> Self { MyEnum::A } // A constructor
    fn print_me(&self) {
        match self {
            MyEnum::A => println!("The first option!"),
            MyEnum::B => println!("The second option!"),
        }
    }
}

fn main() {
    let e = MyEnum::new();
    e.print_me();
}

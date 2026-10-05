struct MyClass {
    a: i32
}

impl MyClass {
    fn print_me(&self) {
        println!("{}", self.a);
    }
}

fn main() {
    let mc = MyClass { a: 42 };
    mc.print_me();
}

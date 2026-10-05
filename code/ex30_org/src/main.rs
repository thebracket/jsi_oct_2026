// Demonstrates the module types from "Program Organization":
// inline modules, file-based modules (goodbye.rs) and directory
// modules (dirmod/mod.rs).
mod goodbye;
mod dirmod;

pub mod private {
    pub fn hi() {
        println!("Say Hi!");
    }
}

pub fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    say_hello();
    private::hi();
    goodbye::bye();
    dirmod::dir_hello();
}

fn main() {
    let i = 5;
    match i {
        5 => {
            println!("5");
            println!("5 is a good number.");
        }
        6 => println!("6"),
        _ => println!("Something else"),
    }
}

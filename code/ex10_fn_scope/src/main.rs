fn main() {
    let i = {
        let mut accumulator = 0;
        for i in 0..10 {
            accumulator += i;
        }
        accumulator
    };
    println!("{i}");
}

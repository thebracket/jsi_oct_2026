fn main() {
    static mut COUNTER: usize = 0;
    std::thread::scope(|scope| {
        for i in 0..3 {
            scope.spawn(|| {
                for _i in 0..100_000 {
                    unsafe {
                        COUNTER += 1;
                    }
                }
            });
        }
    });
    unsafe {
        println!("{COUNTER}");
    }
}

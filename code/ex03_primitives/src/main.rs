use std::mem::size_of;

fn main() {
    let c: i8 = 97;
    let uc: u8 = 10;
    let i: i32 = 123;
    let ui: u32 = 123;
    let s: i16 = 123;
    let us: u16 = 123;
    let l: i64 = 123;
    let ul: u64 = 123;
    let vl: i128 = 123;
    let uvl: u128 = 123;
    let is: isize = 123;
    let us: usize = 123;
    let f: f32 = 123.4;
    let d: f64 = 123.4;

    println!("{c}, {}", size_of::<i8>());
    println!("{uc}, {}", size_of::<u8>());
    println!("{i}, {}", size_of::<i32>());
    println!("{ui}, {}", size_of::<u32>());
    println!("{s}, {}", size_of::<i16>());
    println!("{us}, {}", size_of::<u16>());
    println!("{l}, {}", size_of::<i64>());
    println!("{ul}, {}", size_of::<u64>());
    println!("{vl}, {}", size_of::<i128>());
    println!("{uvl}, {}", size_of::<u128>());
    println!("{is}, {}", size_of::<isize>());
    println!("{us}, {}", size_of::<usize>());
    println!("{f}, {}", size_of::<f32>());
    println!("{d}, {}", size_of::<f64>());
}

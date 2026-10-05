fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let my_vec = vec![1,2,3,4,5];
    println!("{}", sum(&my_vec[0..3]));
}

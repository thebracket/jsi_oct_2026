use std::collections::HashMap;

fn main() {
    let mut my_map = HashMap::new();
    my_map.insert("Hello".to_string(), 5);
    my_map.insert("World".to_string(), 6);

    if let Some(count) = my_map.get("Hello") {
        println!("{count}");
    }
}

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 2);

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The value stored in key 1 is: {}", map[&1]);
    println!("Randomly generated value using rand is: {}", secret_number);
}

use std::collections::HashMap;

fn main() {
    let mapping: HashMap<i32, &str> = HashMap::from([
        (1, "I"),
        (5, "V"),
        (10, "X"),
        (50, "L"),
        (100, "C"),
        (500, "D"),
        (1000, "M"),
    ]);

    println!("{:?}", mapping);
}

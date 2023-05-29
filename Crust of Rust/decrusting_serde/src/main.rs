use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CompundData {
    num: u64,
    str: String,
}

fn main() {
    let obj: CompundData = CompundData {
        num: (20),
        str: ("Hello".to_string()),
    };

    let serialized: String = serde_json::to_string(&obj).unwrap();
    println!("Serialzied: {}", serialized);

    let deserialized: CompundData = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

pub fn raindrops(x: u32) -> String {
    let is_factor = |factor| x % factor == 0;

    let mut res: String = String::new();

    if is_factor(3) {
        res.push_str("Pling");
    }

    if is_factor(5) {
        res.push_str("Plang");
    }

    if is_factor(7) {
        res.push_str("Plong");
    }

    if res.is_empty() {
        res = x.to_string();
    }
    res
}

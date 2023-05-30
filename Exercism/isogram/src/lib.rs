use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let candidate: String = candidate
                                .replace(" ", "")
                                .replace("-", "")
                                .to_lowercase();

    let mut hm: HashMap<char, usize> = HashMap::new();

    for (i, c) in candidate.char_indices() {
        if let Some(_) = hm.insert(c, i) {
            return false;
        }
    }

    return true;
}

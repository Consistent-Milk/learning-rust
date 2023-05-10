struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut max: usize = 0;
        let mut l: usize = 0;

        for (r, c) in s.char_indices() {
            if let Some(end) = hm.insert(c, r) {
                l = usize::max(l, end + 1);
            }
            max = usize::max(r - l + 1, max);
        }
        return max as i32;
    }
}

fn test() -> bool {
    let s: String = String::from("Helllo");

    return Solution::length_of_longest_substring(s) == 3;
}

fn main() {
    let result: bool = test();

    match result {
        true => println!("Test passed."),
        _ => println!("Test failed."),
    }
}

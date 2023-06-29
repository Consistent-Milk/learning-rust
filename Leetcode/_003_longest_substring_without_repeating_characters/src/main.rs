struct Solution;
use std::collections::HashMap;

impl Solution {
    // Algorithm :
    // 1. Initialize an empty mutable HashMap and mutable usize max and l
    // 2. Loop over the string using char_indices() -> Returns index and
    // [char] iterator, i.e., (index, value) as a tuple
    // 3. Define end = hm.insert(key: value, value: index) -> this returns
    // the previously stored value as Option<value> and undates the key with the
    // new index, if the key is not available None is returned.
    // 4. Use a match construct and see if end has a value, or holds None.
    // 5. If end has value => Set l = max(l, end + 1)
    // 6. If end has no value => Do nothing
    // 7. Set max = max(index - l + 1, max)
    // 8. After loop ends, return max

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut max: usize = 0;
        let mut l: usize = 0;

        for (index, value) in s.char_indices() {
            if let Some(end) = hm.insert(value, index) {
                l = usize::max(l, end +1);
            }

            max = usize::max(index - l + 1, max);
        }
        max as i32
    }
}

fn test() -> bool {
    let s: String = String::from("Helllo");

    Solution::length_of_longest_substring(s) == 3
}

fn main() {
    let result: bool = test();

    match result {
        true => println!("Test passed."),
        _ => println!("Test failed."),
    }

    let s: String = String::from("Hello World!");

    let mut hm: HashMap<char, usize> = HashMap::new();

    let mut i: i32 = 0;

    for (index, value) in s.char_indices() {
        let test_end = hm.insert(value, index);

        match test_end {
            Some(test_end) => {
                i += 1;
                println!("At iteration(Some-Arm) ({i}) | Iterating over '{value}' | Current hashmap: {:?}", hm);
                println!("{}", test_end);
            }
            None => {
                i += 1;
                println!("At iteration(None-Arm) ({i}) | Iterating over '{value}' | Current hashmap: {:?}", hm);
            }
        }
        // if let Some(end) = hm.insert(value, index) {
        //     println!("{}", end);
        // }
    }

    println!("{:?}", hm);
}

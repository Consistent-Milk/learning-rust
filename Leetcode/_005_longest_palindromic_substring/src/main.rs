struct Solution;

use std::iter::FromIterator;

impl Solution {
    fn longest_palindrome(s: String) -> String {
        let n: usize = s.len();
        if n == 0 {
            return "".to_string();
        }

        let s: Vec<char> = s.chars().collect();
        let mut start: usize = 0;
        let mut end: usize = 0;

        for i in 0..n {
            let mut left: usize = i;
            let mut right: usize = i;

            while right + 1 < n && s[right + 1] == s[left] {
                right += 1;
            }

            while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
                left -= 1;
                right += 1;
            }

            if right - left > end - start {
                start = left;
                end = right;
            }
        }
        String::from_iter(&s[start..=end])
    }
}

fn test() -> bool {
    let s = "babad".to_string();
    let res = "bab".to_string();
    return Solution::longest_palindrome(s) == res;
}

fn main() {
    let result: bool = test();

    match result {
        true => println!("Test Passed."),
        _ => println!("Test Failed."),
    }
}

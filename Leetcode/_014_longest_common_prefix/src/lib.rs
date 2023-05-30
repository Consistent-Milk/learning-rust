pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }

        let ss: Vec<Vec<char>> = strs.iter().map(|s: &String| s.chars().collect()).collect();

        let n: usize = ss.iter().map(|s: &Vec<char>| s.len()).min().unwrap();

        let mut prefix: Vec<char> = vec![];

        for i in 0..n {
            let c: char = ss[0][i];
            if ss.iter().all(|s: &Vec<char>| s[i] == c) {
                prefix.push(c);
            } else {
                break;
            }
        }

        prefix.iter().collect()
    }
}

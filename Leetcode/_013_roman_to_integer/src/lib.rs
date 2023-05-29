use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut sum: i32 = 0;
        let mut last: i32 = 0;

        for c in s.chars() {
            if let Some(&v) = map.get(&c) {
                if v > last {
                    sum += v - 2 * last;
                } else {
                    sum += v
                }
                last = v;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}

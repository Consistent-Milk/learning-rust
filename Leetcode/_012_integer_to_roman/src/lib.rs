pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let symbols: [(i32, &str); 13] = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut res = String::new();

        for &(value, symbol) in symbols.iter() {
            while num >= value {
                res += symbol;
                num -= value;
            }
            if num == 0 {
                break;
            }
        }

        res
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_3() {
        let num: i32 = 3;
        let res: String = "III".to_string();
        assert_eq!(Solution::int_to_roman(num), res);
    }

    #[test]
    fn test_1994() {
        let num: i32 = 1994;
        let res: String = "MCMXCIV".to_string();
        assert_eq!(Solution::int_to_roman(num), res);
    }
}

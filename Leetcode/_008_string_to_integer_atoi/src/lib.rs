pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // Trim leading whitespace
        let s: &str = s.trim_start();

        // Strip '-' and use match case
        // if it returns Some value then store it in a tuple of (s , -1)
        // If it returns none, that means '-' isn't in the string,
        // strip prefix '+', if '+' isn't available return (s, 1)
        let (s, sign) = match s.strip_prefix('-') {
            Some(s) => (s, -1),
            None => (s.strip_prefix('+').unwrap_or(s), 1),
        };

        s.chars()
            .map(|c: char| c.to_digit(10))
            .take_while(Option::is_some)
            .flatten()
            .fold(0, |acc: i32, digit: u32| {
                acc.saturating_mul(10).saturating_add(sign * digit as i32)
            })
    }
}

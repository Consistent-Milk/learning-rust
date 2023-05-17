pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        if x < 0 {
            return false;
        }

        let mut reversed: i32 = 0;
        let mut number: i32 = x;

        while number > 0 {
            reversed = reversed * 10 + number % 10;
            number /= 10;
        }

        return x == reversed;
    }
}
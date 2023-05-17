use _009_palindrome_number::Solution;

fn main() {
    let result: bool = Solution::is_palindrome(121);
    println!("Is 121 a palindrome -> {}", result);
}


#[test]
fn test_121() {
    let x: i32 = 121;

    let result: bool = Solution::is_palindrome(x);

    assert_eq!(result, true);
}

#[test]
fn test_neg_121() {
    let x: i32 = -121;

    let result: bool = Solution::is_palindrome(x);

    assert_eq!(result, false);
}

#[test]
fn test_10() {
    let x: i32 = 10;

    let result: bool = Solution::is_palindrome(x);

    assert_eq!(result, false);
}
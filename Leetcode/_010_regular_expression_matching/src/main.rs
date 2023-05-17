use _010_regular_expression_matching::Solution;

fn main() {
    let s: String = "aa".to_string();
    let p: String = "a".to_string();
    let result: bool = Solution::is_match(s, p);
    println!("Expected result is false -> Result is {}", result);
}

#[test]
fn test_1() {
    let s: String = "aa".to_string();
    let p: String = "a".to_string();
    let res: bool = false;
    assert_eq!(Solution::is_match(s, p), res);
}

#[test]
fn test_2() {
    let s: String = "aa".to_string();
    let p: String = "a*".to_string();
    let res: bool = true;
    assert_eq!(Solution::is_match(s, p), res);
}

#[test]
fn test_3() {
    let s: String = "ab".to_string();
    let p: String = ".*".to_string();
    let res: bool = true;
    assert_eq!(Solution::is_match(s, p), res);
}

#[test]
fn test_4() {
    let s: String = "aab".to_string();
    let p: String = "c*a*b".to_string();
    let res: bool = true;
    assert_eq!(Solution::is_match(s, p), res);
}

#[test]
fn test_5() {
    let s: String = "mississippi".to_string();
    let p: String = "mis*is*p*.".to_string();
    let res: bool = false;
    assert_eq!(Solution::is_match(s, p), res);
}
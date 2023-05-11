fn main() {
    match test_main() {
        true => println!("Test Passed."),
        _ => println!("Test Failed."),
    }
}

fn test_main() -> bool {
    return _002_reverse_string::reverse("abcd") == String::from("dcba");
}

#[test]
fn test() {
    assert_eq!(_002_reverse_string::reverse("abcd"), String::from("dcba"));
}

fn main() {
    match test_main() {
        true => println!("Test Passed."),
        _ => println!("Test Failed."),
    }
}

fn test_main() -> bool {
    return _001_hello_world::hello() == "Hello, World!";
}

#[test]
fn test() {
    assert_eq!(_001_hello_world::hello(), "Hello, World!");
}

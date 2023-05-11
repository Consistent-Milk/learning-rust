use std::fs::File;
fn main() {
    _9_error_handling::ErrorHandling::with_match("Hello.txt");
    _9_error_handling::ErrorHandling::with_result("NewFile.txt");
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt is not included in this project");
    // Should throw error
    match _9_error_handling::ErrorHandling::read_username_from_file("ello.txt") {
        Ok(name) => println!("Username read successfully {name}"),
        Err(err) => println!("Couldn't read from file {:?}", err),
    };

    match _9_error_handling::ErrorHandling::read_username_from_file_short("ello.txt") {
        Ok(name) => println!("Username read successfully {name}"),
        Err(err) => println!("Couldn't read from file {:?}", err),
    };

    match _9_error_handling::ErrorHandling::read_username_from_file_shortest("ello.txt") {
        Ok(name) => println!("Username read successfully {name}"),
        Err(err) => println!("Couldn't read from file {:?}", err),
    };
}

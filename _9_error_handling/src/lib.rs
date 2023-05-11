use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
pub struct ErrorHandling;

impl ErrorHandling {
    // These functions return nothing, just tries to open a file
    // and if the file is not available creates the file
    pub fn with_match(file_name: &'static str) {
        let greeting_file_result = File::open(file_name);

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(file_name) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }

    pub fn with_result(file_name: &'static str) {
        let _greeting_file_result = File::open(file_name).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(file_name).unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    pub fn read_username_from_file(file_name: &'static str) -> Result<String, io::Error> {
        let username_file_result: Result<File, io::Error> = File::open(file_name);

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username: String = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => return Ok(username),
            Err(e) => return Err(e),
        }
    }

    pub fn read_username_from_file_short(file_name: &'static str) -> Result<String, io::Error> {
        // Using '?' at the end causes a panic if an error occurs
        // otherwise returns the value of a Result
        let mut username_file: File = File::open(file_name)?;
        let mut username: String = String::new();

        // Panics if fails to store read data as string inside
        // username variable
        username_file.read_to_string(&mut username)?;

        // At this point we know that the function didn't panic and
        // the username was read sucessfully so we return Ok()
        return Ok(username);
    }

    pub fn read_username_from_file_shortest(file_name: &'static str) -> Result<String, io::Error> {
        let mut username = String::new();

        File::open(file_name)?.read_to_string(&mut username)?;

        return Ok(username);
    }
}

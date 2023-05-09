fn main() {
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };
    println!("\nThe value of y is: {y}");
    another_function();
    let five_return: i32 = five();
    let five_return_explicit: i32 = five_explicit();
    let plus_one_return: i32 = plus_one(10);
    println!("\nThe function five() returns implicitly: {five_return}");
    println!("\nThe function five() returns explicitly: {five_return_explicit}");
    println!("\nThe function plus_one() returns implicitly: {plus_one_return}");
    print!("\n");
}

fn another_function() {
    println!("Another function.");
}

// A function returns the last expression implicitly
// if a return statement is not used.
// Implicit return must end without a semicolon ';'
fn five() -> i32 {
    5
}

// 6 is unreachable as the functions returns 5 and exits the functional scope
// This will result in a compilation warning but not a 'panic'
fn five_explicit() -> i32 {
    return 5;
    6
}

fn plus_one(x: i32) -> i32 {
    println!("Input value given to plus_one(): {x}");
    x + 1
}

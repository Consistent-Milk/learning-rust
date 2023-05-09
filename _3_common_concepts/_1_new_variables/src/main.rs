fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of constant THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
    shadowing(10);
    shadowing_type_change();
}

fn shadowing(_x: i32) {
    let _x = _x + 1;
    {
        let _x = _x + 2;
        println!("The value of x in inner scope is: {_x}");
    }
    println!("The value of x is: {_x}");
}

fn shadowing_type_change() {
    let spaces = "       ";
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}

use std::io;

fn main() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let a: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = a.0;
    let six_point_four = a.1;
    let one = a.2;

    println!("{five_hundred} {six_point_four} {one}");
    arr_index();
}

// fn arr() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     let a = [3; 5];
// }

fn arr_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

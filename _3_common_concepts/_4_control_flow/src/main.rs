fn main() {
    println!("\n");

    let number: i32 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("\n");

    let condition: bool = true;
    let n: i32 = if condition { 5 } else { 6 };

    println!("The value of n is: {n}");

    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    println!("\n");

    // Outer loop always sets remaining to 10
    // outer loop is labeled 'counting_up and its break statement is inside
    // the inner loop
    // If you didn't want to use loop labeling then the same result
    // can be achieved by moving the if/break statement of outer loop outside
    // the scope of the inner loop
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("\n");

    // Using while allows us to write more compact code without
    // the use of loop/if/break statements
    let mut num = 3;

    while num != 0 {
        println!("{num}!");
        num -= 1;
    }

    println!("LIFTOFF!!!");

    println!("\n");
    iterate_explicit();

    println!("\n");
    iterate_implicit();

    println!("\n");
    for_liftoff();

    println!("\n");
    let fahrenheit: f64 = 100.0;

    let celsius: f64 = fahrenheit_to_celsius(fahrenheit);

    println!("{fahrenheit} fahrenheit = {celsius:.2} degree celsius");

    println!("\n");
}

// This can be done if we know the size of array beforehand
// otherwise we might cause a 'panic' by trying to access
// an out of bounds value or we might not access all of the values
// by setting a higher or lower threshold for index respectively
fn iterate_explicit() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("(Explicit)The value at index {index} is: {}", a[index]);

        index += 1;
    }
}

// This is safer as we are guarranted to access all values
// of the array and also we are sure that we will not access out of bounds value
fn iterate_implicit() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: i32 = 0;

    for element in a {
        println!("(Implicit)The value at index {index} is {element}.");
        index += 1;
    }
}

// (1..4) is a Range which generates 1,2,3 but not 4
// rev() method reverses this range and instead generates 3,2,1
fn for_liftoff() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!");
}

fn fahrenheit_to_celsius(x: f64) -> f64 {
    let y: f64 = (x - 32.0) * (5.0 / 9.0);
    y
}

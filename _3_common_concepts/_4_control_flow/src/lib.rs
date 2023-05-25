pub fn fahrenheit_to_celsius(x: f64) -> f64 {
    let y: f64 = (x - 32.0) * (5.0 / 9.0);
    y
}

/// Returns (n+1)th Fibonacci number
pub fn fib(n: u8) -> u64 {
    let mut v: Vec<u64> = vec![0, 1];

    if n == 0 || n == 1 {
        return v[n as usize];
    }

    for i in 2..=(n as usize) {
        v.push(v[i - 1] + v[i - 2]);
    }

    return v[n as usize];
}

// This can be done if we know the size of array beforehand
// otherwise we might cause a 'panic' by trying to access
// an out of bounds value or we might not access all of the values
// by setting a higher or lower threshold for index respectively
pub fn iterate_explicit() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("(Explicit)The value at index {index} is: {}", a[index]);

        index += 1;
    }
}

// This is safer as we are guarranted to access all values
// of the array and also we are sure that we will not access out of bounds value
pub fn iterate_implicit() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: i32 = 0;

    for element in a {
        println!("(Implicit)The value at index {index} is {element}.");
        index += 1;
    }
}

// (1..4) is a Range which generates 1,2,3 but not 4
// rev() method reverses this range and instead generates 3,2,1
pub fn for_liftoff() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!");
}

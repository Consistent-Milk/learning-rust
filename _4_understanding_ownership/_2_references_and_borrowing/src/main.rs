fn main() {
    let s1: String = String::from("hello");

    let len: usize = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);

    let mut s2: String = String::from("Hello");

    println!("The value of s2 before mutating through reference: {s2}");

    change_mutable_reference(&mut s2);

    println!("The value of s2 after mutating through reference: {s2}");

    let mut s3: String = String::from("Hello");

    // A value can have only one mutable reference
    let _r1: &mut String = &mut s3;
    // Compiling the code the next two line commented out will throw an error
    // let _r2: &mut String = &mut s3;
    // println!("{}, {}", _r1, _r2);
}

// s is a reference to a String
// After function returns s goes out of scope
// but s being a reference does not have ownership
// of what it refers to. Thus whatever it is referring to
// is not dropped as s goes out of scope.
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_mutable_reference(s: &mut String) {
    s.push_str(", World!");
}

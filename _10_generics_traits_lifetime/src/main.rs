use _10_generics_traits_lifetime::*;
fn main() {
    let num_list: Vec<i32> = vec![10, 20, 30];
    let num_list_copy: Vec<i32> = num_list.clone();

    // This is a reference to the largest number in num_list
    let result_proper: &i32 = largest_proper(&num_list);

    // This IS the largest number in num_list
    // num_list had to be cloned because calling the fn largest_proper borrows from
    // num_list and thus num_list is dropped when the function is called and
    // is passed on to the functional scope of largest_proper
    // this passed list also goes out of scope as the function returns value
    let result: i32 = largest(num_list_copy);

    println!("The largest number in num_list using fn largest is: {result}");
    println!("The largest number in num_list using fn largest proper is: {result_proper}");

    let char_list: Vec<char> = vec!['a', 'b', 'c', 'd'];

    let result_char: &char = largest_generic(&char_list);

    println!(
        "The largest character value in char_list is: {}",
        result_char
    );

    // Point<T, U> allows us to take two generic types
    // i.e, we can combine different data types using this syntax
    let p: Point<i32, f64> = Point { x: 5, y: 2.5 };
    let q: Point<f64, f64> = Point { x: 3.0, y: 4.0 };

    println!("p.x = {} and p.y = {}", p.x(), p.y());
    println!(
        "The distance of point q from origin is: {:.2}",
        q.distance_from_origin()
    );
}

#[test]
fn test() {
    let num_list: Vec<i32> = vec![10, 20, 30];
    assert_eq!(_10_generics_traits_lifetime::largest(num_list), 30 as i32);
}

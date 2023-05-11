use rust_math::Algorithms;
fn main() {
    let v = vec![1, 2, 3, 4, 10];

    let s = String::from("Hello");
    let mut s_vec: Vec<char> = Vec::new();

    for c in s.chars() {
        s_vec.push(c);
    }

    let result = Algorithms::largest(&v);
    let result_s = Algorithms::largest(&s_vec);

    println!("The largest element in v is {result}");

    println!("The largest character element in s in {result_s}");
}

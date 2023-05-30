fn main() {
    let num: i32 = 0;

    let binding = num.to_string();
    let num_string: Vec<&str> = binding.split("").collect();
    let digits = num_string.len();

    let mut num_digits: Vec<i32> = Vec::new();

    for i in 1..digits - 1 {
        let val: i32 = num_string[i].parse().unwrap();
        num_digits.push(val);
    }

    let mut sum: i32 = 0;

    for val in num_digits.iter() {
        sum += i32::pow(*val, (digits - 2) as u32);
    }

    println!("{}", sum);
}

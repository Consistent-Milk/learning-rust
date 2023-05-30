pub fn is_armstrong_number(num: u32) -> bool {
    let num: u64 = u64::from(num);
    let binding: String = num.to_string();
    let num_string: Vec<&str> = binding.split("").collect();
    let digits: usize = num_string.len();

    let mut num_digits: Vec<u64> = Vec::new();

    for i in 1..digits - 1 {
        let val: u64 = num_string[i].parse().unwrap();
        num_digits.push(val);
    }

    let mut sum: u64 = 0;

    for val in num_digits.iter() {
        sum += u64::pow(*val, (digits - 2) as u32);
    }

    return sum == num;
}

// pub fn is_armstrong_number(num: u32) -> bool {
//     if num == 0 {
//         return true;
//     }

//     let num = u64::from(num);
//     let digit_count = (num as f64).log10() as u32 + 1;
//     let mut candidate = num as u64;
//     let mut total = 0;

//     while candidate > 0 {
//         total += (candidate % 10).pow(digit_count);
//         candidate /= 10;
//     }

//     num == total
// }

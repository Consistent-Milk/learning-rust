use _008_string_to_integer_atoi::*;
fn main() {
    let s1: String = String::from("42");
    let s1_copy: String = s1.clone();

    let s2: String = String::from("   -42");
    let s2_copy: String = s2.clone();

    let s3: String = String::from("4193 with words");
    let s3_copy: String = s3.clone();

    let s4: String = String::from("0032");
    let s4_copy: String = s4.clone();

    let result1: i32 = Solution::my_atoi(s1);
    let result2: i32 = Solution::my_atoi(s2);
    let result3: i32 = Solution::my_atoi(s3);
    let result4: i32 = Solution::my_atoi(s4);

    println!(" \"{}\" is parsed into {}", s1_copy, result1);
    println!(" \"{}\" is parsed into {}", s2_copy, result2);
    println!(" \"{}\" is parsed into {}", s3_copy, result3);
    println!(" \"{}\" is parsed into {}", s4_copy, result4);
}

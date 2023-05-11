struct Solution;

use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = hm.get(&(target - num)) {
                return vec![j, i as i32];
            } else {
                hm.insert(num, i as i32);
            }

            // let check: Option<&i32> = hm.get(&(target - num));

            // match check {
            //     Some(&j) => return vec![j, i as i32],
            //     None => (),
            // }

            // hm.insert(num, i as i32);
        }
        return vec![-1, -1];
    }
}

fn test() -> bool {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    return Solution::two_sum(nums, target) == vec![0, 1];
}

fn main() {
    let result: bool = test();

    match result {
        true => println!("Test passed."),
        _ => println!("Test failed."),
    }
}

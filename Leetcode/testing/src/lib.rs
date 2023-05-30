use std::cmp::Ordering::{Equal, Greater, Less};
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut closest: i32 = nums[0] + nums[1] + nums[2];
        let n: usize = nums.len();

        for (i, v) in nums.iter().enumerate() {
            let mut left_pointer: usize = i + 1;
            let mut right_pointer: usize = n - 1;

            while left_pointer < right_pointer {
                let sum: i32 = v + nums[left_pointer] + nums[right_pointer];
                let current_difference: i32 = (target - closest).abs();
                let probable_closest: i32 = target - sum;

                if probable_closest.abs() < current_difference {
                    closest = probable_closest;

                    if closest == target {
                        return closest;
                    }
                }

                match (probable_closest).cmp(&0) {
                    Greater => {
                        left_pointer += 1;
                    }

                    Less => {
                        right_pointer -= 1;
                    }

                    Equal => {
                        left_pointer += 1;
                    }
                }
            }
        }

        closest
    }
}

#[test]
fn test() {
    let nums: Vec<i32> = vec![-1, 2, 1, -4];
    let target: i32 = 1;
    let res: i32 = 2;
    assert_eq!(Solution::three_sum_closest(nums, target), res);
}

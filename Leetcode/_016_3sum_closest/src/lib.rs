#![allow(clippy::comparison_chain)]
use std::cmp::Ordering;
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut closest: i32 = nums[0] + nums[1] + nums[2];
        let mut lp: usize;
        let mut rp: usize;

        let n: usize = nums.len();

        if n == 3 {
            return closest;
        }

        for index in nums.iter() {
            let i: usize = *index as usize;
            lp = i + 1;
            rp = nums.len() - 1;

            while rp > lp {
                let total: i32 = nums[i] + nums[lp] + nums[rp];
                let temp: i32 = target - total;

                if (temp).abs() < (target - closest).abs() {
                    closest = total;
                    if closest == target {
                        return closest;
                    }
                }

                match (temp).cmp(&0) {
                    Ordering::Equal => {
                        lp += 1;
                    }

                    Ordering::Greater => {
                        lp += 1;
                    }

                    Ordering::Less => {
                        rp -= 1;
                    }
                }
            }
        }

        closest
    }

    pub fn three_sum_closest_best(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        
        let mut lp: usize;
        let mut rp: usize;
        let mut closest: i32 = nums[0] + nums[1] + nums[2];
        let n: usize = nums.len();

        if n == 3 {
            return closest;
        }

        for (i, num) in nums.iter().enumerate() {
            lp = i + 1;
            rp = n - 1;
        
            while rp > lp {
                let total: i32 = num + nums[lp] +nums[rp];
                let temp: i32 = target - total;

                if (temp).abs() < (target-closest).abs() {
                    closest = total;
                    if closest == target {
                        return closest;
                    }
                }
        
                if (temp) > 0 {
                    lp += 1
                }
                else if (temp) < 0 {
                    rp -= 1
                }
                else {
                    lp += 1;
                }
            }
        }
        closest
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let nums: Vec<i32> = vec![-1, 2, 1, -4];
        let target: i32 = 1;
        let res: i32 = 2;
        assert_eq!(Solution::three_sum_closest(nums, target), res);
    }
}

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let n: usize = nums.len();
        
        if n < 3 {
            return res;
        }

        nums.sort_unstable();

        for i in 0..n - 2 {
            let a: i32 = nums[i];

            if a > 0 {
                break;
            }

            if i > 0 && a == nums[i - 1] {
                continue;
            }

            let mut j: usize = i + 1;
            let mut k: usize = n - 1;

            while j < k {
                let b: i32 = nums[j];
                let c: i32 = nums[k];

                let sum: i32 = a + b + c;

                match sum.cmp(&0) {
                    Ordering::Equal => {
                        res.push(vec![a, b, c]);
                        j += 1;
                        k -= 1;

                        while j < k && nums[j] == nums[j - 1] {
                            j += 1;
                        }

                        while j < k && nums[k] == nums[k + 1] {
                            k -= 1;
                        }
                    }

                    Ordering::Greater => {
                        k -= 1;
                    }

                    Ordering::Less => {
                        j += 1;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
        let res: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(Solution::three_sum(nums), res);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![-2, 0, 1, 1, 2];
        let res: Vec<Vec<i32>> = vec![vec![-2, 0, 2], vec![-2, 1, 1]];
        assert_eq!(Solution::three_sum(nums), res);
    }
}
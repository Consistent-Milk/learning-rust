use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = vec![];
        let len: usize = nums.len();

        if len < 4 {
            return combinations;
        }

        let mut nums: Vec<i32> = nums;
        nums.sort_unstable();

        for i in 0..len - 3 {
            if nums[i] as i64 * 4 > target as i64 {
                break;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            for j in i + 1..len - 2 {
                if nums[j] as i64 * 3 > target as i64 - nums[i] as i64 {
                    continue;
                }

                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let (mut left, mut right) = (j + 1, len - 1);

                while left < right {
                    let cur_val: i64 =
                        nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;
                    match cur_val.cmp(&(target as i64)) {
                        Ordering::Equal => {
                            combinations.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                            left += 1;
                            right -= 1;

                            while left < right && nums[left] == nums[left - 1] {
                                left += 1;
                            }
                            while left < right && nums[right] == nums[right + 1] {
                                right -= 1;
                            }
                        }
                        Ordering::Less => left += 1,
                        Ordering::Greater => right -= 1,
                    }
                }
            }
        }
        combinations
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
        let target: i32 = 0;
        let expected: Vec<Vec<i32>> =
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];

        let result: Vec<Vec<i32>> = Solution::four_sum(nums, target);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![2, 2, 2, 2, 2];
        let target: i32 = 8;
        let expected: Vec<Vec<i32>> = vec![vec![2, 2, 2, 2]];

        let result: Vec<Vec<i32>> = Solution::four_sum(nums, target);

        assert_eq!(expected, result);
    }
}

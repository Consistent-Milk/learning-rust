pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;

        while l < r {
            max = i32::max(i32::min(height[l], height[r])* (r-l) as i32, max);

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        return max;
    }
}
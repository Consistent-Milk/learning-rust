struct Solution;

impl Solution {
    fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort();
        
        let l: usize = nums1.len();

        match l {
            0 => return 0.0,
            1 => return nums1[0] as f64,
            _ => {}
        }

        if l % 2 == 0 {
            let half: usize = l / 2;
            let n1: f64 = nums1[half - 1] as f64;
            let n2: f64 = nums1[half] as f64;
            return (n1 + n2) / 2f64;
        } else {
            return nums1[l / 2] as f64;
        }
    }
}

fn test() -> bool {
    let nums1: Vec<i32> = vec![1, 2];
    let nums2: Vec<i32> = vec![3, 4];
    let expected_result: f64 = 2.5;

    let thresh: f64 = f64::powi(10.0, -10);
    
    let result: f64 = Solution::find_median_sorted_arrays(nums1, nums2);

    println!("Computed median was {}", result);

    return f64::abs(result - expected_result) < thresh;
}

fn main() {
    let result: bool = test();

    match result {
        true => println!("Test Passed."),
        _ => println!("Test Failed."),
    }
}

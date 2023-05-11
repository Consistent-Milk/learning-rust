struct Solution;

impl Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        if n1 < n2 {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        let mut lo = 0;
        let mut hi = n2 * 2;
        while lo <= hi {
            let mid2 = (lo + hi) / 2;
            let mid1 = n1 + n2 - mid2;
            let l1 = if mid1 == 0 {
                std::i32::MIN
            } else {
                nums1[(mid1 - 1) / 2]
            };
            let l2 = if mid2 == 0 {
                std::i32::MIN
            } else {
                nums2[(mid2 - 1) / 2]
            };
            let r1 = if mid1 == n1 * 2 {
                std::i32::MAX
            } else {
                nums1[mid1 / 2]
            };
            let r2 = if mid2 == n2 * 2 {
                std::i32::MAX
            } else {
                nums2[mid2 / 2]
            };

            if l1 > r2 {
                lo = mid2 + 1;
            } else if l2 > r1 {
                hi = mid2 - 1;
            } else {
                return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
            }
        }
        panic!()
    }
}

fn test() -> bool {
    let nums1: Vec<i32> = vec![1, 2];
    let nums2: Vec<i32> = vec![3, 4];
    let res: f64 = 2.5;

    let thresh: f64 = f64::powi(10.0, -10);

    return f64::abs(Solution::find_median_sorted_arrays(nums1, nums2) - res) < thresh;
}

fn main() {
    let result: bool = test();

    match result {
        true => println!("Test Passed."),
        _ => println!("Test Failed."),
    }
}

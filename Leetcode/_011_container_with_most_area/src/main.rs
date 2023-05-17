use _011_container_with_most_area::Solution;

fn main() {
    let height: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    
    let result: i32 = Solution::max_area(height);

    println!("Max area should be 49, result returned is {}", result);
}


#[test]
fn test() {
    let height: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let expected: i32 = 49;

    assert_eq!(Solution::max_area(height), expected);
}
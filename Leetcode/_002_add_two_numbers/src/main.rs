use _002_add_two_numbers::*;

fn test() -> bool {
    let l1: Option<Box<ListNode>> = list!(2, 4, 3);
    let l2: Option<Box<ListNode>> = list!(5, 6, 4);
    let l3: Option<Box<ListNode>> = list!(7, 0, 8);

    Solution::add_two_numbers(l1, l2) == l3
}

fn main() {
    let result = test();

    match result {
        true => println!("Test passed."),
        _ => println!("Test failed."),
    }
}

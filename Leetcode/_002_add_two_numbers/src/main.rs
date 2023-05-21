// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($e:expr) => {
        ListLink::link($e, None)
    };
    ($e:expr, $($tail:tt)*) => {
        ListLink::link($e, list!($($tail)*))
    };
}

pub type ListLink = Option<Box<ListNode>>;

pub trait ListMaker {
    fn link(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode { val, next }))
    }
}

impl ListMaker for ListLink {}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;


impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sum: Option<Box<ListNode>> = None;
        let mut p1: &Option<Box<ListNode>> = &l1;
        let mut p2: &Option<Box<ListNode>> = &l2;

        let mut p3: &mut Option<Box<ListNode>> = &mut sum;

        let mut carry: i32 = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut val: i32 = carry;

            if let Some(n1) = p1.as_ref() {
                val += n1.val;
                p1 = &n1.next;
            }

            if let Some(n2) = p2.as_ref() {
                val += n2.val;
                p2 = &n2.next;
            }

            carry = val / 10;
            *p3 = Some(Box::new(ListNode::new(val % 10)));
            p3 = &mut p3.as_mut().unwrap().next;
        }
        return sum;
    }

    pub fn _add_two_numbers_test(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next),
                    }))
                } else {
                    let carry: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(
                            Solution::add_two_numbers(carry, n1.next),
                            n2.next,
                        ),
                    }))
                }
            }
        }
    }
}

fn test() -> bool {
    let l1: Option<Box<ListNode>> = list!(2, 4, 3);
    let l2: Option<Box<ListNode>> = list!(5, 6, 4);
    let l3: Option<Box<ListNode>> = list!(7, 0, 8);

    return Solution::add_two_numbers(l1, l2) == l3;
}

fn main() {
    let result = test();

    match result {
        true => println!("Test passed."),
        _ => println!("Test failed."),
    }
}

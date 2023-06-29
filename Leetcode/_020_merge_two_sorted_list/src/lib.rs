#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        let mut p1 = l1.unwrap();
        let mut p2 = l2.unwrap();
        
        if p1.val < p2.val {
            p1.next = Self::merge_two_lists(p1.next, Some(p2));
            Some(p1)
        } else {
            p2.next = Self::merge_two_lists(Some(p1), p2.next);
            Some(p2)
        }
    }
}

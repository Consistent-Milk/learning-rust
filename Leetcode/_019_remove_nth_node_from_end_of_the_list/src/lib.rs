// Definition for singly-linked list.
#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
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
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
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

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: ListLink, n: i32) -> ListLink {
        let mut v: Vec<ListLink> = vec![];

        while let Some(mut node) = head {
            head = node.next.take();
            v.push(Some(node));
        }

        let mut res: ListLink = None;

        for (i, link) in v.into_iter().rev().enumerate() {
            if i != (n - 1) as usize {
                let mut node: Box<ListNode> = link.unwrap();
                node.next = res;
                res = Some(node);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let head: ListLink = list!(1, 2, 3, 4, 5);
        let res: ListLink = list!(1, 2, 3, 5);
        let n = 2;
        assert_eq!(Solution::remove_nth_from_end(head, n), res);
    }
}

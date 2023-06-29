impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut v: Vec<Option<Box<ListNode>>> = vec![];

        // 1 -> 2 -> 3 -> None
        // node = 1, head = node.next.take() = 2
        // v.push(Some(node)) -> v = [Some(1)]
        while let Some(mut node) = head {
            head = node.next.take();
            v.push(Some(node));
        }

        let mut res: Option<Box<ListNode>> = None;

        // res = None
        //      0  1  2
        // v = [3, 2 ,1], n = 2, i.e., remove 2 and return List(1 -> 3)
        // 
        // i = 0 => 
        //      node = link.unwrap() = 3
        //      node.next = res = None
        //      res = Some(node) = 3;
        // i = 1 => Skip as i != n - 1 = 2 - 1 = 1 is a condition
        // i = 2 => 
        //      node = link.unwrap() = 1
        //      node.next = res = 3
        //      res = Some(node) = 1
        //
        // return res = 1 -> 3 
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

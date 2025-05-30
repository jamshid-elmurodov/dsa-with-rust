impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;

        while let Some(node) = head {
            let new_node = Box::new(ListNode {
                val: node.val,
                next: res,
            });
            
            res = Some(new_node);
            head = node.next;
        }

        res
    }
}

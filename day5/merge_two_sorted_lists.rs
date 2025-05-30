impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>, 
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let (Some(n1), Some(n2)) = (&l1, &l2) {
            if n1.val < n2.val {
                let next = l1.as_mut().unwrap().next.take();
                tail.next = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                tail.next = l2;
                l2 = next;
            }

            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if l1.is_some() { l1 } else { l2 };

        dummy.next
    }
}


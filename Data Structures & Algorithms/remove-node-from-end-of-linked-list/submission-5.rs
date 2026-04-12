impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();

        // 1. Move fast pointer n steps ahead
        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        // 2. Move both until fast reaches the last node
        let mut fast_ptr = fast.next;
        while let Some(node) = fast_ptr {
            fast_ptr = node.next;
            slow = slow.next.as_mut().unwrap();
        }

        //???
        let next_node = slow.next.as_mut().unwrap().next.take();
        slow.next = next_node;

        dummy.next
    }
}
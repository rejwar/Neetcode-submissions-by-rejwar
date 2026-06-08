impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while let (Some(mut node1), Some(mut node2)) = (list1.take(), list2.take()) {
            if node1.val < node2.val {
                list1 = node1.next.take();
                tail.next = Some(node1);
            } else {
                list2 = node2.next.take();
                tail.next = Some(node2);
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = list1.or(list2);
        dummy.next
    }
}
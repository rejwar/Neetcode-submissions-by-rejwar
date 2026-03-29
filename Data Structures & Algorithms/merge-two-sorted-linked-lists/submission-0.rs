impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                let next = list1.as_mut().unwrap().next.take();
                curr.next = list1;
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                curr.next = list2;
                list2 = next;
            }
            curr = curr.next.as_mut().unwrap();
        }

        curr.next = if list1.is_some() { list1 } else { list2 };
        dummy.next
    }
}
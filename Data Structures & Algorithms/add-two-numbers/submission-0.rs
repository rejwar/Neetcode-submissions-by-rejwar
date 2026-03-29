impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
        }

        dummy.next
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();

        let mut curr1 = l1.as_ref();
        while let Some(node) = curr1 {
            stack1.push(node.val);
            curr1 = node.next.as_ref();
        }

        let mut curr2 = l2.as_ref();
        while let Some(node) = curr2 {
            stack2.push(node.val);
            curr2 = node.next.as_ref();
        }

        let mut carry = 0;
        let mut head = None;

        while !stack1.is_empty() || !stack2.is_empty() || carry != 0 {
            let mut sum = carry;
            if let Some(val) = stack1.pop() {
                sum += val;
            }
            if let Some(val) = stack2.pop() {
                sum += val;
            }

            carry = sum / 10;
            let mut new_node = Box::new(ListNode::new(sum % 10));
            new_node.next = head;
            head = Some(new_node);
        }

        head
    }
}
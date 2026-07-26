impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while let (Some(f), Some(s)) = (fast, slow) {
            if let Some(n) = &f.next {
                slow = &s.next;
                fast = &n.next;
            } else {
                break;
            }
        }

        slow.clone()
    }
}
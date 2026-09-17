impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut slow = &head;
        let mut fast = &head;

        while let (Some(s), Some(f)) = (slow, fast) {
            slow = &s.next;
            if let Some(next_f) = &f.next {
                fast = &next_f.next;
            } else {
                return false;
            }

            if let (Some(s_node), Some(f_node)) = (slow, fast) {
                if std::ptr::eq(s_node.as_ref(), f_node.as_ref()) {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}
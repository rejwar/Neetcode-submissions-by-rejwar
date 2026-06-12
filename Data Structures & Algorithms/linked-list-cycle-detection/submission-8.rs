impl Solution {
    pub fn has_cycle(head: *const ListNode) -> bool {
        let mut slow = head;
        let mut fast = head;

        unsafe {
            loop {
                // Fast hits null → no cycle
                if fast.is_null() || (*fast).next.is_null() {
                    return false;
                }

                slow = (*slow).next;
                fast = (*(*fast).next).next;

                if slow == fast {
                    return true;
                }
            }
        }
    }
}
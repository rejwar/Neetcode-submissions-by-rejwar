impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }

        let mut len = 0;
        let mut ptr = &head;
        while let Some(node) = ptr {
            len += 1;
            ptr = &node.next;
        }

        let k = k % len;
        if k == 0 {
            return head;
        }

        let mut new_tail = &mut head;
        for _ in 0..(len - k - 1) {
            new_tail = &mut new_tail.as_mut().unwrap().next;
        }

        let mut new_head = new_tail.as_mut().unwrap().next.take();

        let mut tail = &mut new_head;
        while tail.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        *tail = head;

        new_head
    }

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;

        for _ in 0..(left - 1) {
            prev = &mut prev.as_mut().unwrap().next;
        }

        let mut curr = prev.as_mut().unwrap().next.take();
        let mut reversed = None;

        for _ in 0..=(right - left) {
            if let Some(mut node) = curr {
                curr = node.next.take();
                node.next = reversed;
                reversed = Some(node);
            }
        }

        let mut tail = &mut reversed;
        while tail.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        *tail = curr;

        prev.as_mut().unwrap().next = reversed;

        dummy.unwrap().next
    }}

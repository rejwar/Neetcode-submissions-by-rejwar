impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut len = 0;
        let mut curr = head.as_ref();
        
        while let Some(n) = curr {
            len += 1;
            curr = n.next.as_ref();
        }
        
        if len <= 2 {
            return;
        }
        
        let mut curr = head.as_mut().unwrap();
        for _ in 0..(len - 1) / 2 {
            curr = curr.next.as_mut().unwrap();
        }
        
        let mut l2 = curr.next.take();
        let mut prev = None;
        
        while let Some(mut n) = l2 {
            l2 = n.next.take();
            n.next = prev;
            prev = Some(n);
        }
        
        let mut p1 = head.as_mut();
        let mut p2 = prev;
        
        while let Some(mut n2) = p2 {
            if let Some(n1) = p1 {
                p2 = n2.next.take();
                n2.next = n1.next.take();
                n1.next = Some(n2);
                p1 = n1.next.as_mut().unwrap().next.as_mut();
            }
        }
    }
}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            let next = node.next.take(); // ১. 
            node.next = prev;            // ২. 
            prev = Some(node);           // ৩.
            curr = next;                 // ৪. 
            
        }

        prev
    }
}
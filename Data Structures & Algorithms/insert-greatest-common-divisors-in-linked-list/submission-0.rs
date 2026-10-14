impl Solution {
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.as_mut();
        
        while let Some(node) = curr {
            if let Some(next_node) = node.next.take() {
                let gcd_val = Self::gcd(node.val, next_node.val);
                let mut new_node = Box::new(ListNode::new(gcd_val));
                
                new_node.next = Some(next_node);
                node.next = Some(new_node);
                
                curr = node.next.as_mut().unwrap().next.as_mut();
            } else {
                break;
            }
        }
        
        head
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
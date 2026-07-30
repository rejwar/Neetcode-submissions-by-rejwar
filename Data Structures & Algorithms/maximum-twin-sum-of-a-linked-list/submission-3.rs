impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut vals = Vec::new();
        let mut current = head;
        
        while let Some(node) = current {
            vals.push(node.val);
            current = node.next;
        }
        
        let mut max_sum = 0;
        let n = vals.len();
        
        for i in 0..(n / 2) {
            max_sum = max_sum.max(vals[i] + vals[n - 1 - i]);
        }
        
        max_sum
    }
}
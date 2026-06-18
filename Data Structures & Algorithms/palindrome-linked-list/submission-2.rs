// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut values = Vec::new();
        let mut current = head;
        
        // Traverse the linked list and push values into the vector
        while let Some(node) = current {
            values.push(node.val);
            current = node.next;
        }
        
        // Use Rust's iterators to compare the forward and reverse sequences
        values.iter().eq(values.iter().rev())
    }
}
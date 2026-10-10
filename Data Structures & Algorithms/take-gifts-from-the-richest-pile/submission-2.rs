use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        
        let mut heap: BinaryHeap<i64> = gifts.into_iter().map(|x| x as i64).collect();

        for _ in 0..k {
            if let Some(max_val) = heap.pop() {
                
                let remaining = (max_val as f64).sqrt().floor() as i64;
                heap.push(remaining);
            }
        }

        heap.into_iter().sum()
    
}
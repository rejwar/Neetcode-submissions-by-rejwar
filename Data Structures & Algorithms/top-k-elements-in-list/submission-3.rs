use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut count = HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut bucket: Vec<Vec<i32>> = vec![vec![]; n + 1];
        for (num, freq) in count {
            bucket[freq as usize].push(num);
        }

        let mut res = Vec::new();
        for i in (0..bucket.len()).rev() {
            for num in &bucket[i] {
                res.push(*num);
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        res
    }
}
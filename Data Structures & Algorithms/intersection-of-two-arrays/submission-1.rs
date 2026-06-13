use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // Collect all unique elements from nums1 into a HashSet
        let mut set: HashSet<i32> = nums1.into_iter().collect();
        let mut result = Vec::new();

        // Iterate through nums2 to find intersections
        for num in nums2 {
            // set.remove(&num) returns true if the value was present in the set.
            // By removing it, we ensure that duplicate values in nums2 
            // don't result in duplicates in our result array.
            if set.remove(&num) {
                result.push(num);
            }
        }

        result
    }
}
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            // Skip the same element to avoid duplicate triplets
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len().saturating_sub(1);

            while l < r {
                let three_sum = nums[i] + nums[l] + nums[r];

                if three_sum > 0 {
                    r -= 1;
                } else if three_sum < 0 {
                    l += 1;
                } else {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    
                    // Skip duplicates for the second element
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                }
            }
        }
        res
    }
}
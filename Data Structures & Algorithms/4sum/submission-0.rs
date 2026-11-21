macro_rules! four_sum_impl {
    ($nums:expr, $target:expr) => {{
        let mut nums = $nums;
        let target = $target as i64;
        let n = nums.len();
        let mut res = Vec::new();
        if n < 4 {
            res
        } else {
            nums.sort_unstable();
            for i in 0..n - 3 {
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }
                if (nums[i] as i64) + (nums[i + 1] as i64) + (nums[i + 2] as i64) + (nums[i + 3] as i64) > target {
                    break;
                }
                if (nums[i] as i64) + (nums[n - 3] as i64) + (nums[n - 2] as i64) + (nums[n - 1] as i64) < target {
                    continue;
                }
                for j in i + 1..n - 2 {
                    if j > i + 1 && nums[j] == nums[j - 1] {
                        continue;
                    }
                    if (nums[i] as i64) + (nums[j] as i64) + (nums[j + 1] as i64) + (nums[j + 2] as i64) > target {
                        break;
                    }
                    if (nums[i] as i64) + (nums[j] as i64) + (nums[n - 2] as i64) + (nums[n - 1] as i64) < target {
                        continue;
                    }
                    let (mut l, mut r) = (j + 1, n - 1);
                    while l < r {
                        let sum = (nums[i] as i64) + (nums[j] as i64) + (nums[l] as i64) + (nums[r] as i64);
                        if sum == target {
                            res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                            l += 1;
                            r -= 1;
                            while l < r && nums[l] == nums[l - 1] {
                                l += 1;
                            }
                            while l < r && nums[r] == nums[r + 1] {
                                r -= 1;
                            }
                        } else if sum < target {
                            l += 1;
                        } else {
                            r -= 1;
                        }
                    }
                }
            }
            res
        }
    }};
}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        four_sum_impl!(nums, target)
    }
}
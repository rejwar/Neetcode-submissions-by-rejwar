macro_rules! remove_dups_at_most_two {
    ($nums:expr) => {{
        let len = $nums.len();
        if len <= 2 {
            len as i32
        } else {
            let mut write = 2;
            for read in 2..len {
                if $nums[read] != $nums[write - 2] {
                    $nums[write] = $nums[read];
                    write += 1;
                }
            }
            write as i32
        }
    }};
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        remove_dups_at_most_two!(nums)
    }
}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut insert_pos = 0;
        
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(insert_pos, i);
                insert_pos += 1;
            }
        }
    }
}
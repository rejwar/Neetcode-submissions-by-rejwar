impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let k = k as usize;
        let mut min_ops = k;
        let mut current_whites = 0;

        for i in 0..blocks.len() {
            // Add the current block to the window
            if blocks[i] == b'W' {
                current_whites += 1;
            }

            // Remove the block sliding out of the window
            if i >= k && blocks[i - k] == b'W' {
                current_whites -= 1;
            }

            // Update min_ops once we have a full window of size k
            if i >= k - 1 {
                min_ops = min_ops.min(current_whites);
            }
        }

        min_ops as i32
    }
}

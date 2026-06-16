impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut base_satisfied = 0;
        let mut window_extra = 0;
        let mut max_extra = 0;
        let minutes = minutes as usize;

        for i in 0..customers.len() {
            // 1. Add already satisfied customers to the base total
            if grumpy[i] == 0 {
                base_satisfied += customers[i];
            } else {
                // 2. If grumpy, these customers could be saved by the technique. Add to current window.
                window_extra += customers[i];
            }

            // 3. If our window exceeds the allowed 'minutes', slide the left edge forward
            if i >= minutes {
                let left = i - minutes;
                // Only subtract from the window if they were grumpy (since we only added grumpy ones to window_extra)
                if grumpy[left] == 1 {
                    window_extra -= customers[left];
                }
            }

            // 4. Update the maximum extra customers we've seen so far
            max_extra = std::cmp::max(max_extra, window_extra);
        }

        // Return the guaranteed satisfied customers + the best boost from the secret technique
        base_satisfied + max_extra
    }
}
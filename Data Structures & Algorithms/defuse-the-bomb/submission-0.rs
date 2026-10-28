impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut res = vec![0; n];
        if k == 0 {
            return res;
        }

        // Determine the initial window boundaries [start, end] for index 0
        let (mut start, mut end) = if k > 0 {
            (1, k as usize)
        } else {
            (n - (-k as usize), n - 1)
        };

        // Calculate the initial window sum for index 0
        let mut window_sum: i32 = (start..=end).map(|i| code[i % n]).sum();

        // Slide the window across the circular array
        for i in 0..n {
            res[i] = window_sum;
            window_sum -= code[start % n];
            start += 1;
            end += 1;
            window_sum += code[end % n];
        }

        res
    }
}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut row = vec![1; n];

        for _ in 0..m - 1 {
            let mut new_row = vec![1; n];
            
            for j in (0..n - 1).rev() {
                new_row[j] = new_row[j + 1] + row[j];
            }
            row = new_row;
        }

        row[0]
    }
}
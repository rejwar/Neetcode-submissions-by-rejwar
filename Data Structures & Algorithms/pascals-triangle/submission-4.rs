impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        for n in 0..num_rows as i64 {
            let mut row = vec![1i32];
            let mut val: i64 = 1;
            for k in 1..=n {
                val = val * (n - k + 1) / k;
                row.push(val as i32);
            }
            res.push(row);
        }
        res
    }
}
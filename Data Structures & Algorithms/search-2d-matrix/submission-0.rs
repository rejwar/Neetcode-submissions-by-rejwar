impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut l = 0;
        let mut r = (rows * cols) as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            let row = (m as usize) / cols;
            let col = (m as usize) % cols;

            if matrix[row][col] > target {
                r = m - 1;
            } else if matrix[row][col] < target {
                l = m + 1;
            } else {
                return true;
            }
        }
        false
    }
}
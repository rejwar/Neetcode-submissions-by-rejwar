impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                match (i, j) {
                    (0, 0) => {} // arting cell remains unchanged
                    (0, _) => grid[i][j] += grid[i][j - 1], // Top row: can only come from the left
                    (_, 0) => grid[i][j] += grid[i - 1][j], // Left col: can only come from above
                    _ => grid[i][j] += grid[i - 1][j].min(grid[i][j - 1]), // Minimum of top or left
                }
            }
        }

        grid[m - 1][n - 1]
    }
}
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        fn dfs(r: i32, c: i32, grid: &mut Vec<Vec<char>>) {
            let rows = grid.len() as i32;
            let cols = grid[0].len() as i32;

            if r < 0 || r >= rows || c < 0 || c >= cols || grid[r as usize][c as usize] == '0' {
                return;
            }

            
            grid[r as usize][c as usize] = '0';

            
            dfs(r + 1, c, grid);
            dfs(r - 1, c, grid);
            dfs(r, c + 1, grid);
            dfs(r, c - 1, grid);
        }

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    count += 1;
                    dfs(r as i32, c as i32, &mut grid);
                }
            }
        }

        count
    }
}
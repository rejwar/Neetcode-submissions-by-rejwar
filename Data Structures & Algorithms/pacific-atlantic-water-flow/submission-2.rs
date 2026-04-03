use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if grid.is_empty() { return vec![]; }
        
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut pac = HashSet::new();
        let mut atl = HashSet::new();

        fn dfs(r: i32, c: i32, visit: &mut HashSet<(i32, i32)>, prev_height: i32, grid: &Vec<Vec<i32>>) {
            let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
            if visit.contains(&(r, c)) || r < 0 || c < 0 || r >= rows || c >= cols || grid[r as usize][c as usize] < prev_height {
                return;
            }
            visit.insert((r, c));
            let h = grid[r as usize][c as usize];
            dfs(r + 1, c, visit, h, grid);
            dfs(r - 1, c, visit, h, grid);
            dfs(r, c + 1, visit, h, grid);
            dfs(r, c - 1, visit, h, grid);
        }

        for c in 0..cols {
            dfs(0, c as i32, &mut pac, grid[0][c], &grid);
            dfs((rows - 1) as i32, c as i32, &mut atl, grid[rows - 1][c], &grid);
        }

        for r in 0..rows {
            dfs(r as i32, 0, &mut pac, grid[r][0], &grid);
            dfs(r as i32, (cols - 1) as i32, &mut atl, grid[r][cols - 1], &grid);
        }

        let mut res = vec![];
        for r in 0..rows {
            for c in 0..cols {
                if pac.contains(&(r as i32, c as i32)) && atl.contains(&(r as i32, c as i32)) {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }
        res
    }
}
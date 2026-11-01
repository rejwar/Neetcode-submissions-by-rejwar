use std::collections::VecDeque;

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let m = grid.len();
        if m == 0 {
            return;
        }
        let n = grid[0].len();
        let mut queue = VecDeque::new();

        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 0 {
                    queue.push_back((r, c));
                }
            }
        }

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in dirs {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == i32::MAX {
                        grid[nr][nc] = grid[r][c] + 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
    }
}
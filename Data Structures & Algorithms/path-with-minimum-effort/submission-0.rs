use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let rows = heights.len();
        let cols = heights[0].len();
        let mut min_efforts = vec![vec![i32::MAX; cols]; rows];
        let mut pq = BinaryHeap::new();

        min_efforts[0][0] = 0;
        pq.push(Reverse((0, 0, 0)));

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(Reverse((effort, r, c))) = pq.pop() {
            if r == rows - 1 && c == cols - 1 {
                return effort;
            }

            if effort > min_efforts[r][c] {
                continue;
            }

            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    
                    let next_effort = effort.max((heights[r][c] - heights[nr][nc]).abs());

                    if next_effort < min_efforts[nr][nc] {
                        min_efforts[nr][nc] = next_effort;
                        pq.push(Reverse((next_effort, nr, nc)));
                    }
                }
            }
        }
        
        0
    }
}
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    perimeter += 4;

                    // Subtract 2 for every shared edge (top and left)
                    if r > 0 && grid[r - 1][c] == 1 {
                        perimeter -= 2;
                    }
                    if c > 0 && grid[r][c - 1] == 1 {
                        perimeter -= 2;
                    }
                }
            }
        }

        perimeter
    }
}
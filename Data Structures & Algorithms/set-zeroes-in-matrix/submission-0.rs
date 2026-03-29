impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut row_zero = false;

        // 1. Determine which rows and columns need to be zeroed
        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == 0 {
                    matrix[0][c] = 0; // Use first row as marker
                    if r > 0 {
                        matrix[r][0] = 0; // Use first col as marker
                    } else {
                        row_zero = true;
                    }
                }
            }
        }

        // 2. Iterate through the matrix (excluding first row/col) and zero out cells
        for r in 1..rows {
            for c in 1..cols {
                if matrix[0][c] == 0 || matrix[r][0] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }

        // 3. Zero out the first column if necessary
        if matrix[0][0] == 0 {
            for r in 0..rows {
                matrix[r][0] = 0;
            }
        }

        // 4. Zero out the first row if necessary
        if row_zero {
            for c in 0..cols {
                matrix[0][c] = 0;
            }
        }
    }
}
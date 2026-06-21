struct NumMatrix {
    // We store the prefix sum matrix here
    prefix_sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() || matrix[0].is_empty() {
            return NumMatrix { prefix_sum: vec![] };
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        // Create a (rows + 1) x (cols + 1) matrix initialized with 0s
        let mut prefix_sum = vec![vec![0; cols + 1]; rows + 1];

        for r in 0..rows {
            for c in 0..cols {
                prefix_sum[r + 1][c + 1] = matrix[r][c] 
                    + prefix_sum[r][c + 1] 
                    + prefix_sum[r + 1][c] 
                    - prefix_sum[r][c];
            }
        }

        NumMatrix { prefix_sum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize;
        let c2 = col2 as usize;

        // Use the inclusion-exclusion principle for 2D prefix sums
        self.prefix_sum[r2 + 1][c2 + 1] 
            - self.prefix_sum[r1][c2 + 1] 
            - self.prefix_sum[r2 + 1][c1] 
            + self.prefix_sum[r1][c1]
    }
}
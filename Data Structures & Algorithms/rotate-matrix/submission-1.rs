impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let (mut l, mut r) = (0, matrix.len() - 1);

        while l < r {
            for i in 0..(r - l) {
                let (top, bottom) = (l, r);

                let top_left = matrix[top][l + i];
                matrix[top][l + i] = matrix[bottom - i][l];
                matrix[bottom - i][l] = matrix[bottom][r - i];
                matrix[bottom][r - i] = matrix[top + i][r];
                matrix[top + i][r] = top_left;
            }
            r -= 1;
            l += 1
        }
    }
}
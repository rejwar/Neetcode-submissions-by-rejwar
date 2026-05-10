impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let n_sq = n * n;
        let mut counts = vec![0; n_sq + 1];
        
        for row in grid {
            for num in row {
                counts[num as usize] += 1;
            }
        }
        
        let mut a = 0;
        let mut b = 0;
        
        for i in 1..=n_sq {
            if counts[i] == 2 {
                a = i as i32;
            } else if counts[i] == 0 {
                b = i as i32;
            }
        }
        
        vec![a, b]
    }
}
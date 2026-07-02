impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        
        for op in operations {
            match op.as_str() {
                "+" => {
                    // Record a new score that is the sum of the previous two scores.
                    let n = stack.len();
                    stack.push(stack[n - 1] + stack[n - 2]);
                }
                "D" => {
                    // Record a new score that is double the previous score.
                    stack.push(stack.last().unwrap() * 2);
                }
                "C" => {
                    // Invalidate the previous score, removing it from the record.
                    stack.pop();
                }
                _ => {
                    // Record a new score of that integer.
                    stack.push(op.parse::<i32>().unwrap());
                }
            }
        }
        
        // Return the sum of all the scores on the record.
        stack.iter().sum()
    }
}
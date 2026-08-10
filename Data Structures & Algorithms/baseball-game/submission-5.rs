impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        
        for op in operations {
            match op.as_str() {
                "+" => {
                    let n = stack.len();
                    stack.push(stack[n - 1] + stack[n - 2]);
                }
                "D" => {
                    stack.push(stack.last().unwrap() * 2);
                }
                "C" => {
                    stack.pop();
                }
                _ => {
                    stack.push(op.parse::<i32>().unwrap());
                }
            }
        }
        
        stack.iter().sum()
    }
}
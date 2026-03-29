impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut stack = String::new();

        fn backtrack(open_n: i32, closed_n: i32, n: i32, stack: &mut String, res: &mut Vec<String>) {
            if open_n == n && closed_n == n {
                res.push(stack.clone());
                return;
            }

            if open_n < n {
                stack.push('(');
                backtrack(open_n + 1, closed_n, n, stack, res);
                stack.pop();
            }

            if closed_n < open_n {
                stack.push(')');
                backtrack(open_n, closed_n + 1, n, stack, res);
                stack.pop();
            }
        }

        backtrack(0, 0, n, &mut stack, &mut res);
        res
    }
}
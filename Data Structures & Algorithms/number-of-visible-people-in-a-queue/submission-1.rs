impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut ans = vec![0; n];
        let mut stack: Vec<i32> = Vec::new();

        for i in (0..n).rev() {
            let mut visible = 0;
            
            while let Some(&top) = stack.last() {
                if top < heights[i] {
                    visible += 1;
                    stack.pop();
                } else {
                    visible += 1;
                    break;
                }
            }
            
            ans[i] = visible;
            stack.push(heights[i]);
        }

        ans
    }
}
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut stack: Vec<(usize, i32)> = Vec::new(); 

        for (i, &h) in heights.iter().enumerate() {
            let mut start = i;
            
            while let Some(&(idx, ht)) = stack.last() {
                if ht > h {
                    stack.pop();
                    max_area = max_area.max(ht * (i - idx) as i32);
                    start = idx;
                } else {
                    break;
                }
            }
            stack.push((start, h));
        }

        let n = heights.len();
        for (idx, ht) in stack {
            max_area = max_area.max(ht * (n - idx) as i32);
        }

        max_area
    }
}
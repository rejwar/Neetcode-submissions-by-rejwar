impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_v = 0;
        let (mut l, mut r) = (0, heights.len() - 1);

        while l < r {
            
            let area = (r - l) as i32 * heights[l].min(heights[r]);
            max_v = max_v.max(area);

            
            if heights[l] < heights[r] {
                l += 1;
            } else {
                r -=
            }
        }
        max_v
    }
}
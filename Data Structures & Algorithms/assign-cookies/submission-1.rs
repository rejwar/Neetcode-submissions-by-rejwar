impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        // Sort both arrays (unstable sort is slightly faster and fine for primitives)
        g.sort_unstable();
        s.sort_unstable();
        
        let mut child_i = 0;
        let mut cookie_j = 0;
        
        while child_i < g.len() && cookie_j < s.len() {
            // If the cookie is big enough, the child is content
            if s[cookie_j] >= g[child_i] {
                child_i += 1;
            }
            // Always move to the next cookie (either it was used or it was too small)
            cookie_j += 1;
        }
        
        // The child pointer represents the number of satisfied children
        child_i as i32
    }
}
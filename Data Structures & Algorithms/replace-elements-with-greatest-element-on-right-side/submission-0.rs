impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max_so_far = -1;

        for i in (0..arr.len()).rev() {
            let current = arr[i];       // 
            arr[i] = max_so_far;        // ২. 
            max_so_far = max_so_far.max(current); // ৩.
        }

        arr
    }
}
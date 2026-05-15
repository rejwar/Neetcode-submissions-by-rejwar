impl Solution {
    pub fn find_lucky(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable(); // sort so equal numbers are adjacent
        let n = arr.len();
        let mut lucky = -1;
        let mut i = 0;

        while i < n {
            let val = arr[i];
            let mut count = 0;
            // count occurrences of val using a for loop over a slice
            for j in i..n {
                if arr[j] == val {
                    count += 1;
                } else {
                    break;
                }
            }
            if val == count && val > lucky {
                lucky = val;
            }
            i += count; // jump to next distinct number
        }
        lucky
    }
}

impl Solution {
    pub fn find_lucky(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let n = arr.len();
        let mut lucky = -1;
        let mut i = 0;

        // outer loop over distinct numbers
        for i in 0..n {
            // skip if already processed (i is moved forward manually)
            if i == 0 || arr[i] != arr[i - 1] {
                let val = arr[i];
                let mut count = 0;
                // count how many times val appears
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
            }
        }
        lucky
    }
}
use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let dict: HashSet<&str> = dictionary.iter().map(|w| w.as_str()).collect();
        let n = s.len();
        let mut dp = vec![0; n + 1];

        for i in (0..n).rev() {
            dp[i] = dp[i + 1] + 1;
            for j in (i + 1)..=n {
                if dict.contains(&s[i..j]) {
                    dp[i] = dp[i].min(dp[j]);
                }
            }
        }

        dp[0] as i32
    }
}
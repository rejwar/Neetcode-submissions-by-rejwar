macro_rules! divide_players_logic {
    ($skill:expr) => {{
        let mut s = $skill;
        s.sort_unstable();
        let n = s.len();
        let target = s[0] + s[n - 1];
        let mut total_chemistry: i64 = 0;
        let mut valid = true;

        for i in 0..(n / 2) {
            if s[i] + s[n - 1 - i] != target {
                valid = false;
                break;
            }
            total_chemistry += (s[i] as i64) * (s[n - 1 - i] as i64);
        }

        if valid { total_chemistry } else { -1 }
    }};
}

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        divide_players_logic!(skill)
    }
}
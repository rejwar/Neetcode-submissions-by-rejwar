macro_rules! solve_pairs {
    ($spells:expr, $potions:expr, $success:expr) => {{
        let mut p = $potions;
        p.sort_unstable();
        let m = p.len();
        $spells
            .into_iter()
            .map(|s| {
                let target = ($success + s as i64 - 1) / s as i64;
                let idx = p.partition_point(|&x| (x as i64) < target);
                (m - idx) as i32
            })
            .collect()
    }};
}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        solve_pairs!(spells, potions, success)
    }
}
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut w = 0;
        for g in chars.clone().chunk_by(|a, b| a == b) {
            let s = if g.len() > 1 { format!("{}{}", g[0], g.len()) } else { format!("{}", g[0]) };
            for c in s.chars() { chars[w] = c; w += 1; }
        }
        w as i32
    }
}
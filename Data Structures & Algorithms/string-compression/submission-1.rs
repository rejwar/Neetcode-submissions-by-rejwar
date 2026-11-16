impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (mut i, mut w) = (0, 0);
        macro_rules! p { ($c:expr) => {{ chars[w] = $c; w += 1; }} }
        while i < chars.len() {
            let (c, s) = (chars[i], i);
            while i < chars.len() && chars[i] == c { i += 1; }
            p!(c);
            if i - s > 1 { for d in (i - s).to_string().chars() { p!(d); } }
        }
        w as i32
    }
}
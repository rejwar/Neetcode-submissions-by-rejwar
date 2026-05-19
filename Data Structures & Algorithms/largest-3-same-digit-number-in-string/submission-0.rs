impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.as_bytes()
            .windows(3)
            .filter(|w| w[0] == w[1] && w[1] == w[2])
            .map(|w| w[0])
            .max()
            .map(|b| String::from_utf8(vec![b; 3]).unwrap())
            .unwrap_or_default()
    }
}
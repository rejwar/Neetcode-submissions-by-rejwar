impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut l_rem = 0;
        let mut r_rem = 0;

        // 1. Count exact number of invalid '(' and ')' to remove
        for ch in s.chars() {
            if ch == '(' {
                l_rem += 1;
            } else if ch == ')' {
                if l_rem > 0 {
                    l_rem -= 1;
                } else {
                    r_rem += 1;
                }
            }
        }

        let mut result = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        Self::backtrack(&chars, 0, l_rem, r_rem, &mut result);
        result
    }

    fn backtrack(
        chars: &[char],
        start: usize,
        l_rem: i32,
        r_rem: i32,
        result: &mut Vec<String>,
    ) {
        if l_rem == 0 && r_rem == 0 {
            if Self::is_valid(chars) {
                result.push(chars.iter().collect());
            }
            return;
        }

        for i in start..chars.len() {
            // Skip duplicate adjacent parentheses to avoid generating duplicate strings
            if i > start && chars[i] == chars[i - 1] {
                continue;
            }

            // Prune if remaining characters are fewer than the removals still needed
            if (l_rem + r_rem) as usize > chars.len() - i {
                return;
            }

            // Try removing an extra '('
            if l_rem > 0 && chars[i] == '(' {
                let mut next_chars = chars.to_vec();
                next_chars.remove(i);
                Self::backtrack(&next_chars, i, l_rem - 1, r_rem, result);
            }

            // Try removing an extra ')'
            if r_rem > 0 && chars[i] == ')' {
                let mut next_chars = chars.to_vec();
                next_chars.remove(i);
                Self::backtrack(&next_chars, i, l_rem, r_rem - 1, result);
            }
        }
    }

    fn is_valid(chars: &[char]) -> bool {
        let mut balance = 0;
        for &ch in chars {
            if ch == '(' {
                balance += 1;
            } else if ch == ')' {
                balance -= 1;
                if balance < 0 {
                    return false;
                }
            }
        }
        balance == 0
    }
}
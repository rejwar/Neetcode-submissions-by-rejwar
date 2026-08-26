impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut current_number = 0;
        let mut last_number = 0;
        let mut result = 0;
        let mut sign = b'+';
        
        let bytes = s.as_bytes();
        let n = bytes.len();
        
        for i in 0..n {
            let b = bytes[i];
            
            if b.is_ascii_digit() {
                current_number = current_number * 10 + (b - b'0') as i32;
            }
            
            if (!b.is_ascii_digit() && b != b' ') || i == n - 1 {
                match sign {
                    b'+' => {
                        result += last_number;
                        last_number = current_number;
                    }
                    b'-' => {
                        result += last_number;
                        last_number = -current_number;
                    }
                    b'*' => {
                        last_number *= current_number;
                    }
                    b'/' => {
                        last_number /= current_number;
                    }
                    _ => {}
                }
                sign = b;
                current_number = 0;
            }
        }
        
        result + last_number
    }
}
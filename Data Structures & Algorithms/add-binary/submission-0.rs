impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut i = a_bytes.len() as i32 - 1;
        let mut j = b_bytes.len() as i32 - 1;
        let mut carry = 0;
        let mut result = Vec::new();

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;
            
            if i >= 0 {
                sum += (a_bytes[i as usize] - b'0') as i32;
                i -= 1;
            }
            if j >= 0 {
                sum += (b_bytes[j as usize] - b'0') as i32;
                j -= 1;
            }
            
            result.push(((sum % 2) as u8 + b'0') as char);
            carry = sum / 2;
        }

        result.into_iter().rev().collect()
    }
}
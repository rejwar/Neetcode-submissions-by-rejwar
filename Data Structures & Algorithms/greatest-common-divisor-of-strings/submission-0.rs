impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
            return String::new();
        }
        
        let mut a = str1.len();
        let mut b = str2.len();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        
        str1[..a].to_string()
    }
}
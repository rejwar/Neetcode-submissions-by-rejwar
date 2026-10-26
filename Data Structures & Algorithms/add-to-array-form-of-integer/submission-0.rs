impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut i = num.len() as isize - 1;
        
        while i >= 0 && k > 0 {
            let idx = i as usize;
            let sum = num[idx] + k;
            num[idx] = sum % 10;
            k = sum / 10;
            i -= 1;
        }
        
        let mut prefix = Vec::new();
        while k > 0 {
            prefix.push(k % 10);
            k /= 10;
        }
        
        if !prefix.is_empty() {
            prefix.reverse();
            prefix.extend(num);
            prefix
        } else {
            num
        }
    }
}
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut res: i32 = 0;

        while x != 0 {
            let digit = x % 10;
            x /= 10;

            if let Some(new_res) = res.checked_mul(10).and_then(|r| r.checked_add(digit)) {
                res = new_res;
            } else {
                return 0;
            }
        }

        res
    }
}
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;

        for &n in &nums {
            if n > max1 {
                max2 = max1;
                max1 = n;
            } else if n > max2 {
                max2 = n;
            }

            if n < min1 {
                min2 = min1;
                min1 = n;
            } else if n < min2 {
                min2 = n;
            }
        }

        max1 * max2 - min1 * min2
    }
}
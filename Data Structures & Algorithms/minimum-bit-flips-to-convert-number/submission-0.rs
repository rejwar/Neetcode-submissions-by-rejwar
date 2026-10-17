impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        // XOR gives us a 1 for every bit that is different between start and goal
        // count_ones() counts the number of 1s (set bits) in the resulting binary number
        (start ^ goal).count_ones() as i32
    }
}
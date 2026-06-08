// Forward declaration of guess API.
// unsafe fn guess(num: i32) -> i32;

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        for num in 1..=n {
            if guess(num) == 0 {
                return num;
            }
        }
        n
    }
}
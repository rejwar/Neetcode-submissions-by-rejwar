/** * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 * otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
  
    pub unsafe fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);

        while left <= right {
            let mid = left + (right - left) / 2;
            
            match guess(mid) {
                0 => return mid,
                -1 => right = mid - 1,
                1 => left = mid + 1,
                _ => unreachable!(),
            }
        }

        -1
    }
}
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut count0 = students.iter().filter(|&&s| s == 0).count() as i32;
        let mut count1 = students.len() as i32 - count0;

        for sandwich in sandwiches {
            if sandwich == 0 {
                if count0 > 0 {
                    count0 -= 1;
                } else {
                    return count1;
                }
            } else {
                if count1 > 0 {
                    count1 -= 1;
                } else {
                    return count0;
                }
            }
        }
        0
    }
}
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let mut starts: Vec<i32> = intervals.iter().map(|i| i.start).collect();
        let mut ends: Vec<i32> = intervals.iter().map(|i| i.end).collect();
        starts.sort();
        ends.sort();

        let (mut res, mut e) = (0, 0);
        for s in 0..starts.len() {
            if starts[s] < ends[e] {
                res += 1;
            } else {
                e += 1;
            }
        }
        res
    }
}
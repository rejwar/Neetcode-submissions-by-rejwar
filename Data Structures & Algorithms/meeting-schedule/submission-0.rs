impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
        intervals.sort_by_key(|i| i.start);
        for i in 0..intervals.len().saturating_sub(1) {
            if intervals[i].end > intervals[i + 1].start {
                return false;
            }
        }
        true
    }
}
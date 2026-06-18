impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        // Calculate the minute hand angle (6 degrees per minute)
        let minute_angle = minutes as f64 * 6.0;
        
        // Calculate the hour hand angle (30 degrees per hour + 0.5 degrees per minute)
        let hour_angle = (hour % 12) as f64 * 30.0 + (minutes as f64 * 0.5);
        
        // Find the absolute difference between the two angles
        let diff = (hour_angle - minute_angle).abs();
        
        // The problem asks for the smaller angle, so we return the minimum 
        // between the calculated difference and the remaining inner angle.
        diff.min(360.0 - diff)
    }
}
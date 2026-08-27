impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        // Combine position and speed into pairs
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        
        // Sort cars by position in descending order (closest to target first)
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut fleets = 0;
        let mut max_time = 0.0;

        for (p, s) in cars {
            // Calculate time to reach the target as a float
            let time = (target - p) as f64 / s as f64;
            
            // If the current car's time is strictly greater than the max time of the 
            // fleet ahead, it cannot catch up and forms a new fleet.
            if time > max_time {
                fleets += 1;
                max_time = time;
            }
        }

        fleets
    }
}
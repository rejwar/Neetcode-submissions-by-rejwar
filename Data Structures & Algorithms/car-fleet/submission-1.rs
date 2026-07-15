impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        cars.sort_unstable_by_key(|&(p, _)| std::cmp::Reverse(p));

        let mut fleets = 0;
        let mut max_time = 0.0;

        for (p, s) in cars {
            let time = (target - p) as f64 / s as f64;
            if time > max_time {
                fleets += 1;
                max_time = time;
            }
        }

        fleets
    }
}
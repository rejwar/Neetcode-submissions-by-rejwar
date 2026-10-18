use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        // Map each stop to the indices of the buses that visit it
        let mut stop_to_buses: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, route) in routes.iter().enumerate() {
            for &stop in route {
                stop_to_buses.entry(stop).or_default().push(i);
            }
        }

        let mut queue = VecDeque::new();
        let mut visited_buses = vec![false; routes.len()];
        let mut visited_stops = HashSet::new();

        visited_stops.insert(source);
        
        // Initialize the queue with all buses that stop at the source
        if let Some(buses) = stop_to_buses.get(&source) {
            for &bus in buses {
                visited_buses[bus] = true;
                queue.push_back(bus);
            }
        }

        let mut buses_taken = 1;

        // BFS to find the shortest path (minimum buses)
        while !queue.is_empty() {
            let size = queue.len();
            
            for _ in 0..size {
                let bus = queue.pop_front().unwrap();
                
                for &stop in &routes[bus] {
                    if stop == target {
                        return buses_taken;
                    }
                    
                    // If
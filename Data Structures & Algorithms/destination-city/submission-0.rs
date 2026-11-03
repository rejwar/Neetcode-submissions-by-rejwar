use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut starts = HashSet::new();

        // S
        for path in &paths {
            starts.insert(&path[0]);
        }

        // Step 2: 
        for path in &paths {
            let destination = &path[1];
            if !starts.contains(destination) {
                return destination.clone();
            }
        }

        String::new()
    }
}
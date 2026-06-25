use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn most_visited_pattern(
        username: Vec<String>,
        timestamp: Vec<i32>,
        website: Vec<String>,
    ) -> Vec<String> {
        // Step 1: Group websites by user and pair them with their timestamps
        let mut user_visits: HashMap<String, Vec<(i32, String)>> = HashMap::new();
        for i in 0..username.len() {
            user_visits
                .entry(username[i].clone())
                .or_default()
                .push((timestamp[i], website[i].clone()));
        }

        let mut pattern_counts: HashMap<Vec<String>, i32> = HashMap::new();

        // Step 2: Sort each user's visits by time and extract unique 3-sequences
        for (_, mut visits) in user_visits {
            // Sort by timestamp to ensure correct chronological order
            visits.sort_by_key(|v| v.0);

            let n = visits.len();
            let mut user_patterns: HashSet<Vec<String>> = HashSet::new();

            // Generate all possible 3-sequence combinations for the user
            for i in 0..n {
                for j in (i + 1)..n {
                    for k in (j + 1)..n {
                        let pattern = vec![
                            visits[i].1.clone(),
                            visits[j].1.clone(),
                            visits[k].1.clone(),
                        ];
                        user_patterns.insert(pattern);
                    }
                }
            }

            // Step 3: Count the unique sequences across all users
            for pattern in user_patterns {
                *pattern_counts.entry(pattern).or_insert(0) += 1;
            }
        }

        // Step 4: Find the pattern with the max count (and lexicographically smallest if tied)
        let mut max_count = 0;
        let mut best_pattern: Vec<String> = Vec::new();

        for (pattern, count) in pattern_counts {
            if count > max_count || (count == max_count && pattern < best_pattern) {
                max_count = count;
                best_pattern = pattern;
            }
        }

        best_pattern
    }
}
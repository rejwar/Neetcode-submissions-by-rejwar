impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut total_time = 0;
        let k_usize = k as usize;
        let target_tickets = tickets[k_usize];

        for (i, &t) in tickets.iter().enumerate() {
            if i <= k_usize {
                // People at or in front of k will buy at most `target_tickets`
                total_time += t.min(target_tickets);
            } else {
                // People behind k will buy at most `target_tickets - 1`
                total_time += t.min(target_tickets - 1);
            }
        }

        total_time
    }
}
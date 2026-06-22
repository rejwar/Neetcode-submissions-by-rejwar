impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        // WRONG: Ignores everyone else in the line and the 1-ticket-at-a-time rule.
        tickets[k as usize]
    }
}

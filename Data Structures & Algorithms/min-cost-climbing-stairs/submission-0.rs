impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {

        for i in (0..cost.len() - 2).rev() {
            cost[i] += std::cmp::min(cost[i + 1], cost[i + 2]);
        }

        std::cmp::min(cost[0], cost[1])
    }
}
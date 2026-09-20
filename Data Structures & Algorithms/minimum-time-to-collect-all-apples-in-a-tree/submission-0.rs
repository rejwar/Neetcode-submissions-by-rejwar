impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut adj = vec![vec![]; n as usize];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        fn dfs(node: usize, parent: usize, adj: &[Vec<usize>], has_apple: &[bool]) -> i32 {
            let mut total_time = 0;

            for &child in &adj[node] {
                if child == parent {
                    continue;
                }

                let child_time = dfs(child, node, adj, has_apple);

                if child_time > 0 || has_apple[child] {
                    total_time += child_time + 2;
                }
            }

            total_time
        }

        dfs(0, 0, &adj, &has_apple)
    }
}
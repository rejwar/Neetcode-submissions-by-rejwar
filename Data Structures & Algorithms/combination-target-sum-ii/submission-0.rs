// impl Solution {
//     pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

//     }
// }

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        candidates.sort();

        fn backtrack(
            idx: usize,
            target: i32,
            candidates: &Vec<i32>,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 {
                res.push(path.clone());
                return;
            }
            if target < 0 {
                return;
            }

            for i in idx..candidates.len() {
                if i > idx && candidates[i] == candidates[i - 1] {
                    continue;
                }
                path.push(candidates[i]);
                backtrack(i + 1, target - candidates[i], candidates, path, res);
                path.pop();
            }
        }

        backtrack(0, target, &candidates, &mut path, &mut res);
        res
    }
}
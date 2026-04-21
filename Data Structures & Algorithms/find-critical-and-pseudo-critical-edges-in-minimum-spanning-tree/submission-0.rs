struct DSU {
    parent: Vec<usize>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            components: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
            self.components -= 1;
            true
        } else {
            false
        }
    }
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        

        let mut edges_with_idx: Vec<(usize, usize, i32, usize)> = edges
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e[0] as usize, e[1] as usize, e[2], i))
            .collect();
        edges_with_idx.sort_unstable_by_key(|&(_, _, w, _)| w);

     
        let get_mst_weight = |ignore_idx: i32, force_idx: i32| -> i32 {
            let mut dsu = DSU::new(n);
            let mut weight = 0;

            if force_idx != -1 {
                let (u, v, w, _) = edges_with_idx[force_idx as usize];
                dsu.union(u, v);
                weight += w;
            }

            for (i, &(u, v, w, _)) in edges_with_idx.iter().enumerate() {
                if i as i32 == ignore_idx { continue; } 
                
                if dsu.union(u, v) {
                    weight += w;
                }
            }

            
            if dsu.components > 1 { i32::MAX } else { weight }
        };

       
        let base_weight = get_mst_weight(-1, -1);

        let mut critical = Vec::new();
        let mut pseudo = Vec::new();

  
        for i in 0..edges_with_idx.len() {
            let original_idx = edges_with_idx[i].3 as i32;
            
           
            if get_mst_weight(i as i32, -1) > base_weight {
                critical.push(original_idx);
            } 
            else if get_mst_weight(-1, i as i32) == base_weight {
                pseudo.push(original_idx);
            }
        }

        vec![critical, pseudo
    }
}
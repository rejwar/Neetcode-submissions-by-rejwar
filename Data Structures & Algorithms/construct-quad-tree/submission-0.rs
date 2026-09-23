use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
        let n = grid.len();
        Self::build(&grid, 0, 0, n)
    }

    fn build(grid: &Vec<Vec<i32>>, r: usize, c: usize, n: usize) -> Option<Rc<RefCell<Node>>> {
        let first_val = grid[r][c];
        let mut is_leaf = true;

        // Check 
        for i in r..(r + n) {
            for j in c..(c + n) {
                if grid[i][j] != first_val {
                    is_leaf = false;
                    break;
                }
            }
            if !is_leaf {
                break;
            }
        }

        if is_leaf {
           
            Some(Rc::new(RefCell::new(Node {
                val: first_val == 1,
                is_leaf: true,
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None,
            })))
        } else {
            // Recursive case: 
            let half = n / 2;
            Some(Rc::new(RefCell::new(Node {
                val: true, // v
                is_leaf: false,
                top_left: Self::build(grid, r, c, half),
                top_right: Self::build(grid, r, c + half, half),
                bottom_left: Self::build(grid, r + half, c, half),
                bottom_right: Self::build(grid, r + half, c + half, half),
            })))
        }
    }
}
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if node.is_none() {
            return None;
        }

        let mut old_to_new = HashMap::new();

        fn dfs(
            node: Rc<RefCell<Node>>,
            old_to_new: &mut HashMap<i32, Rc<RefCell<Node>>>,
        ) -> Rc<RefCell<Node>> {
            let val = node.borrow().val;

            if let Some(cloned) = old_to_new.get(&val) {
                return cloned.clone();
            }

            let copy = Rc::new(RefCell::new(Node::new(val)));
            old_to_new.insert(val, copy.clone());

            for neighbor in &node.borrow().neighbors {
                let cloned_neighbor = dfs(neighbor.clone(), old_to_new);
                copy.borrow_mut().neighbors.push(cloned_neighbor);
            }

            copy
        }

        Some(dfs(node.unwrap(), &mut old_to_new))
    }
}
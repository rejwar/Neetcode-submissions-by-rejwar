use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            map.insert(val, i as i32);
        }

        fn build(
            left: i32, 
            right: i32, 
            postorder: &mut Vec<i32>, 
            map: &HashMap<i32, i32>
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if left > right {
                return None;
            }

            let val = postorder.pop().unwrap();
            let mut node = TreeNode::new(val);
            let idx = *map.get(&val).unwrap();

            node.right = build(idx + 1, right, postorder, map);
            node.left = build(left, idx - 1, postorder, map);

            Some(Rc::new(RefCell::new(node)))
        }

        let len = inorder.len() as i32;
        build(0, len - 1, &mut postorder, &map)
    }
}
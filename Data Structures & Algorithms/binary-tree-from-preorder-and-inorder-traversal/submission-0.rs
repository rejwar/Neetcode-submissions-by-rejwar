use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_map = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            inorder_map.insert(val, i);
        }
        
        let mut preorder_idx = 0;
        Self::build(&preorder, &inorder_map, 0, inorder.len() as i32 - 1, &mut preorder_idx)
    }
    
    fn build(
        preorder: &[i32], 
        map: &HashMap<i32, usize>, 
        left: i32, 
        right: i32, 
        idx: &mut usize
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        
        let val = preorder[*idx];
        *idx += 1;
        
        let mut node = TreeNode::new(val);
        let mid = *map.get(&val).unwrap() as i32;
        
        node.left = Self::build(preorder, map, left, mid - 1, idx);
        node.right = Self::build(preorder, map, mid + 1, right, idx);
        
        Some(Rc::new(RefCell::new(node)))
    }
}
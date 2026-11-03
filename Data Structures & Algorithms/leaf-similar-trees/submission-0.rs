use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>, 
        root2: Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        let mut leaves1 = Vec::new();
        let mut leaves2 = Vec::new();
        
        Self::get_leaves(root1, &mut leaves1);
        Self::get_leaves(root2, &mut leaves2);
        
        leaves1 == leaves2
    }
    
    fn get_leaves(node: Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
        if let Some(curr) = node {
            let borrowed = curr.borrow();
            
          
            if borrowed.left.is_none() && borrowed.right.is_none() {
                leaves.push(borrowed.val);
            }
            
  
            Self::get_leaves(borrowed.left.clone(), leaves);
            Self::get_leaves(borrowed.right.clone(), leaves);
        }
    }
}
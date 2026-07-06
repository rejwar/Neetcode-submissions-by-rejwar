impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        
        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                let n_borrowed = n.borrow();
                traverse(&n_borrowed.left, res);
                res.push(n_borrowed.val);
                traverse(&n_borrowed.right, res);
            }
        }
        
        traverse(&root, &mut result);
        result
    }
}
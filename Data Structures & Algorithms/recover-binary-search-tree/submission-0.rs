use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first = None;
        let mut second = None;
        let mut prev = None;

        Self::inorder(root, &mut prev, &mut first, &mut second);

        if let (Some(f), Some(s)) = (first, second) {
            let mut f_borrow = f.borrow_mut();
            let mut s_borrow = s.borrow_mut();
            std::mem::swap(&mut f_borrow.val, &mut s_borrow.val);
        }
    }

    fn inorder(
        node: &Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>
    ) {
        if let Some(n) = node {
            let n_borrow = n.borrow();
            
            Self::inorder(&n_borrow.left, prev, first, second);

            if let Some(p) = prev {
                if p.borrow().val > n_borrow.val {
                    if first.is_none() {
                        *first = Some(p.clone());
                    }
                    *second = Some(n.clone());
                }
            }
            *prev = Some(n.clone());

            Self::inorder(&n_borrow.right, prev, first, second);
        }
    }
}
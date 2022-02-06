use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
        val,
        left: None,
        right: None
        }
    }

    pub fn dup(&self) -> Self {
        let left = Self::dup_option(&self.left);
        let right = Self::dup_option(&self.right);
        TreeNode {
            val: self.val,
            left,
            right
        }
    }

    pub fn dup_option(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(n) => {
                Some(Rc::new(RefCell::new(n.as_ref().borrow().dup())))
            }
        }
    }
}
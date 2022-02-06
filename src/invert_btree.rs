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
}

pub struct Solution;

impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root.take() {
            None => None,
            Some(node) => {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                node.borrow_mut().left = Self::invert_tree(right);
                node.borrow_mut().right = Self::invert_tree(left);
                Some(node)
            }
        }
    }
}

pub fn tester(root: Option<Rc<RefCell<TreeNode>>>, expected: Option<Rc<RefCell<TreeNode>>>) {
    assert_eq!(Solution::invert_tree(root), expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
        })));
        tester(root1, root2);
    }

    #[test]
    fn test2() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
        })));
        tester(root1, root2);
    }
}
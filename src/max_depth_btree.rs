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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut head = root;
        match head.take() {
            None => 0,
            Some(node) => {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                Self::max_depth(left).max(Self::max_depth(right)) + 1
            }
        }
    }
}

pub fn tester(root: Option<Rc<RefCell<TreeNode>>>, expected: i32) {
    assert_eq!(Solution::max_depth(root), expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                })))
            })))
        })));
        tester(root, 3);
    }
}
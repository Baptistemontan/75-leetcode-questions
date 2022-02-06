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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut pmut = p;
        let mut qmut = q;
        match (pmut.take(), qmut.take()) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(node1), Some(node2)) => {
                if node1.borrow().val == node2.borrow().val {
                    let left1 = node1.borrow_mut().left.take();
                    let right1 = node1.borrow_mut().right.take();
                    let left2 = node2.borrow_mut().left.take();
                    let right2 = node2.borrow_mut().right.take();
                    Self::is_same_tree(left1, left2) && Self::is_same_tree(right1, right2)
                } else {
                    false
                }
            }
        }
    }
}

pub fn tester(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>, expected: bool) {
    assert_eq!(Solution::is_same_tree(root1, root2), expected);
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
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        })));
        tester(root1, root2, true);
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
        tester(root1, root2, false);
    }
}
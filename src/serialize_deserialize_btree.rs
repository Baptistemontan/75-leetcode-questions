use crate::treenode::*;

use std::rc::Rc;
use std::cell::RefCell;


struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
impl Codec {
    fn new() -> Self {
        Codec {

        }
    }

    fn serialize(&self, mut root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root.take() {
            None => String::from("{None}"),
            Some(node) => {
                let left = self.serialize(node.borrow_mut().left.take());
                let right = self.serialize(node.borrow_mut().right.take());
                let val = node.borrow().val;
                format!("{{val:{},left:{},right:{}}}", val, left, right)
            }
        }
    }

    fn find_end_index(s: &str) -> usize {
        let mut depth = 0;
        for (i, c) in s.char_indices() {
            match c {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {},
            }
            if depth == 0 {
                return i;
            }
        }
        s.len()
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data == "{None}" {
            return None;
        }
        let val_start = data.char_indices().find(|(_, c)| c == &':').unwrap().0 + 1;
        let val_end = data.char_indices().find(|(_, c)| c == &',').unwrap().0;
        let val = i32::from_str_radix(&data[val_start..val_end], 10).unwrap();
        let left_start = val_end + 6;
        let left_end = Self::find_end_index(&data[left_start..]) + left_start;
        let right_start = left_end + 8;
        let right_end = Self::find_end_index(&data[right_start..]) + right_start;
        let left = self.deserialize(String::from(&data[left_start..=left_end]));
        let right = self.deserialize(String::from(&data[right_start..=right_end]));
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left,
            right
        })))
    }
}

pub fn tester(root: Option<Rc<RefCell<TreeNode>>>) {
    let codec = Codec::new();
    let data = codec.serialize(TreeNode::dup_option(&root));
    assert_eq!(codec.deserialize(data), root);
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
        tester(root);
    }
}
use std::iter::FromIterator;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNodeHead {
    root: Option<Box<ListNode>>
}

impl ListNodeHead {
    pub fn new(root: Option<Box<ListNode>>) -> Self {
        ListNodeHead {
            root,
        }
    }

    pub fn push(&mut self, val: i32) {
        let next = self.root.take();
        self.root.replace(Box::new(ListNode {
            val,
            next
        }));
    }

    pub fn take(self) -> Option<Box<ListNode>> {
        self.root
    }
}

impl FromIterator<i32> for ListNodeHead {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut list: Option<Box<ListNode>> = None;
        let mut last: Option<*mut ListNode> = None;
        for v in iter.into_iter() {
            let mut new_node = Box::new(ListNode::new(v));
            let new_ptr = Some(&mut *new_node as *mut ListNode);
            match last.take() {
                None => {
                    last = new_ptr;
                    list.replace(new_node);
                },
                Some(last_ptr) => {
                    unsafe {
                        (*last_ptr).next.replace(new_node);
                    }
                    last = new_ptr;
                }
            }
        }
        ListNodeHead {
            root: list
        }
    }
}

pub struct ListNodeIter {
    root: Option<Box<ListNode>>
}

impl Iterator for ListNodeIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.root.take() {
            None => None,
            Some(new_node) => {
                let val = new_node.val;
                self.root = new_node.next;
                Some(val)
            }
        }
    }
}

impl IntoIterator for ListNodeHead {
    type Item = i32;
    type IntoIter = ListNodeIter;
    fn into_iter(self) -> Self::IntoIter {
        ListNodeIter { root: self.root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v: Vec<i32> = vec![2, 4, 8, 0, 5];
        let a: ListNodeHead = v.clone().into_iter().collect();
        let b: Vec<i32> = a.into_iter().collect();
        assert_eq!(v, b);
    }
}
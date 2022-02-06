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

pub struct Solution;

pub fn reverter(root: Option<Box<ListNode>>, end: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match root {
        Some(mut list) => {
            let next = list.next.take();
            list.next = end;
            reverter(next, Some(list))
        },
        None => end
    }
}

pub fn from_arr(arr: Vec<i32>) -> Option<Box<ListNode>> {
    arr.into_iter().rev().fold(None, |next, val| Some(Box::new(ListNode {
        val,
        next
    })))
}

pub fn to_arr(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut top = head;
    let mut arr = vec![];
    loop {
        match top.take() {
            Some(node) => {
                top = node.next;
                arr.push(node.val);
            },
            _ => break,
        } 
    }
    arr
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut top = head;
        let mut end = None;
        loop {
            match top.take() {
                Some(mut node) => {
                    let next = node.next;
                    node.next = end.take();
                    end = Some(node);
                    top = next;
                },
                _ => break,
            }
        }
        end
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse1() {
        let input = vec![12, 5, 8, 0];
        let mut reversed_arr = input.clone();
        reversed_arr.reverse();
        let result = to_arr(Solution::reverse_list(from_arr(input)));
        assert_eq!(result, reversed_arr);
    }
}
use crate::linked_list::*;

pub struct Solution;


impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let len = Self::length(&head);

        let mut ptr = head.as_mut();
        for _ in 0..(len / 2) {
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        if let Some(node) = ptr {
            let reverse = Self::reverse(node.next.take());

            Self::merge(head, reverse);
        }
    }

    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }

    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

    fn merge(mut left_head: &mut Option<Box<ListNode>>, mut right_head: Option<Box<ListNode>>) {
        loop {
            match (left_head.as_mut(), right_head.take()) {
                (Some(left), Some(mut right)) => {
                    let next = left.next.take();
                    right_head = right.next.take();
                    right.next = next;
                    left.next = Some(right);
                    left_head = &mut left.next.as_mut().unwrap().next
                },
                _ => break
            }
        }
    }
}
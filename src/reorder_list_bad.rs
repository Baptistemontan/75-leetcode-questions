use crate::linked_list::*;

pub struct Solution;


impl Solution {
    fn split_mid(mut head: Option<&mut Box<ListNode>>) -> (Option<&mut Box<ListNode>>, Option<Box<ListNode>>) {
        match head {
            Some(ref node) => {
                match node.next.as_ref() {
                    None => {
                        return (head, None);
                    },
                    _ => ()
                }
            },
            None => {
                return (None, None);
            }
        }
        let mut slow: *mut ListNode = &mut ***head.as_mut().unwrap();
        let mut fast: *mut ListNode = &mut ***head.as_mut().unwrap();
        loop {
            let mut next = unsafe {
                &mut *fast
            }.next.as_mut();
            if next.is_none() || next.as_ref().unwrap().next.is_none() {
                break;
            }
            fast = next.as_mut().unwrap().next.as_mut().unwrap().as_mut();
            slow = unsafe{
                &mut *slow
            }.next.as_mut().unwrap().as_mut();
        }
        let mid = unsafe {
            &mut *slow
        }.next.take();
        return (head, mid);
        // damn splitting linked list is a pein in rust
        // surely by doing a first pass to find length then splitting at the middle would be easier to do
        // be this is (should be?) faster
    }

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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


    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let (mut head1, head2) = Self::split_mid(head.as_mut());
        let mut head2 = Self::reverse_list(head2);
        // head1 size == head2 size or head1 size = head2 size + 1
        while head2.is_some() {
            let current_node = head1.take().unwrap();
            let next_node = current_node.next.take();
            let mut new_next_node = head2.take();
            let new_head = new_next_node.as_mut().unwrap().next.take();
            new_next_node.as_mut().unwrap().next = next_node;
            current_node.next = new_next_node;
            head1 = current_node.next.as_mut().unwrap().next.as_mut();
            head2 = new_head;
        }
    }

    // alright I don't even know how it worked but well I love rust, it worked on leetcode on the first try without testing it
    // not quite the speed efficienty I hoped for but meh, linked list in rust are a pain
}
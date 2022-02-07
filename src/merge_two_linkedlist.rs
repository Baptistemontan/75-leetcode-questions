use crate::linked_list::*;

pub struct Solution;




impl Solution {
    fn merger(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {     
        if list2.is_none() {
            return list1;
        }

        let mut iter: *mut ListNode = match &mut list1 {
            None => return list2,
            Some(list) => {
                &mut **list as *mut ListNode
            }
        };

        while list2.is_some() {
            let mut current_node = unsafe {
                &mut *iter
            };
            if current_node.next.is_none() {
                current_node.next = list2.take();
                break;
            }
            let current_node2 = list2.as_ref().unwrap();
            let next_node = current_node.next.as_mut().unwrap();
            if current_node.val <= current_node2.val && current_node2.val <= next_node.val { // .. -> cn -> cn2 -> cn.next
                let mut current_node2 = list2.take().unwrap();
                list2 = current_node2.next.take();
                current_node2.next = current_node.next.take();
                current_node.next.replace(current_node2);
                
            }
            iter = &mut **current_node.next.as_mut().unwrap() as *mut ListNode;
        }
        list1
    }

    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1.take(), list2.take()) {
            (None, None) => None,
            (Some(list), None) | (None, Some(list)) => Some(list),
            (Some(list1), Some(list2)) => {
                if list1.val < list2.val {
                    Self::merger(Some(list1), Some(list2))
                } else {
                    Self::merger(Some(list2), Some(list1))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tester(mut v1: Vec<i32>, mut v2: Vec<i32>) {
        v1.sort();
        v2.sort();
        let list1: ListNodeHead = v1.clone().into_iter().collect();
        let list2: ListNodeHead = v2.clone().into_iter().collect();

        v1.append(&mut v2);
        v1.sort();

        let res: Vec<i32> = ListNodeHead::new(Solution::merge_two_lists(list1.take(), list2.take())).into_iter().collect();

        assert_eq!(res, v1);
    }

    #[test]
    fn test1() {
        tester(vec![1,2,4], vec![1,3,4]);
    }

    #[test]
    fn test2() {
        tester(vec![1], vec![]);
    }

    #[test]
    fn test3() {
        tester(vec![], vec![]);
    }
}


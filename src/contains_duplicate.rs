pub struct Solution;

use std::{collections::{HashSet}};

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut s: HashSet<i32> = HashSet::new();
        for i in nums {
            if s.contains(&i) {
                return true;
            }
            s.insert(i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1,2,3,1];
        let expect = true;
        assert_eq!(Solution::contains_duplicate(nums), expect);
    }

    #[test]
    fn test2() {
        let nums = vec![1,2,3,4];
        let expect = false;
        assert_eq!(Solution::contains_duplicate(nums), expect);
    }

    #[test]
    fn test3() {
        let nums = vec![1,1,1,3,3,4,3,2,4,2];
        let expect = true;
        assert_eq!(Solution::contains_duplicate(nums), expect);
    }
}
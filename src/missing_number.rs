pub struct Solution;


impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let size = nums.len() as i32;
        nums.iter().fold(0, |acc, n| acc ^ n) ^ match size % 4 {
            0 => size,
            1 => 1,
            2 => size + 1,
            _ => 0
        }
    }
}

pub fn tester(nums: Vec<i32>, expected: i32) {
    assert_eq!(Solution::missing_number(nums), expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        tester(vec![3,0,1], 2);
    }

    #[test]
    fn test2() {
        tester(vec![0,1], 2);
    }

    #[test]
    fn test3() {
        tester(vec![9,6,4,2,3,5,7,0,1], 8);
    }
}
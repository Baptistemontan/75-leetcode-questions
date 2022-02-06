pub struct Solution;

pub fn compare_result(res: Vec<i32>, expected: Vec<i32>) -> bool {
    if res == expected {
        true
    } else if res[0] == expected[1] && res[1] == expected[0] {
        true
    } else {
        println!("bad result");
        println!("result: {:?}", res);
        println!("expect: {:?}", expected);
        false
    }
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut sorted = nums.clone();
        sorted.sort();

        let mut left = 0;
        let mut right = sorted.len() - 1;
        loop {
            let sum = sorted[left] + sorted[right];
            if sum == target {
                let (i, _) = nums.iter().enumerate().find(|(_, val)| **val == sorted[left]).unwrap();
                let (j, _) = nums.iter().enumerate().find(|(i2, val)| **val == sorted[right] && *i2 != i).unwrap();
                return vec![i as i32, j as i32];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let expect = vec![0,1];
        let result = Solution::two_sum(nums, target);
        assert!(compare_result(result, expect));
    }

    #[test]
    fn test2() {
        let nums = vec![3,2,4];
        let target = 6;
        let expect = vec![1,2];
        let result = Solution::two_sum(nums, target);
        assert!(compare_result(result, expect));
    }

    #[test]
    fn test3() {
        let nums = vec![3,3];
        let target = 6;
        let expect = vec![0,1];
        let result = Solution::two_sum(nums, target);
        assert!(compare_result(result, expect));
    }

    #[test]
    fn test4() {
        let nums = vec![-1,-2,-3,-4,-5];
        let target = -8;
        let expect = vec![2,4];
        let result = Solution::two_sum(nums, target);
        assert!(compare_result(result, expect));
    }

}
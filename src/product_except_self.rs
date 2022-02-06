pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut answer1 = vec![1; len];
        let mut answer2 = vec![1; len];
        for i in 1..len {
            let j = len - i - 1;
            answer1[i] = answer1[i - 1] * nums[i - 1];
            answer2[j] = answer2[j + 1] * nums[j + 1];
        }
        answer1.iter().zip(answer2.iter()).map(|(a, b)| a * b).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1,2,3,4];
        let expected = vec![24,12,8,6];
        assert_eq!(Solution::product_except_self(nums), expected);
    }

    #[test]
    fn test2() {
        let nums = vec![-1,1,0,-3,3];
        let expected = vec![0,0,9,0,0];
        assert_eq!(Solution::product_except_self(nums), expected);
    }
}
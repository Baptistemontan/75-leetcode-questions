pub struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        (0..u32::BITS).map(|i| 1 << i)
            .map(|mask| n & mask)
            .fold(0, |acc, i| if i != 0 {acc + 1} else {acc})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input_str = "00000000000000000000000000001011";
        let input = u32::from_str_radix(input_str, 2).unwrap();
        let expected = 3;
        assert_eq!(Solution::hammingWeight(input), expected);
    }

    #[test]
    fn test2() {
        let input_str = "00000000000000000000000010000000";
        let input = u32::from_str_radix(input_str, 2).unwrap();
        let expected = 1;
        assert_eq!(Solution::hammingWeight(input), expected);
    }

    #[test]
    fn test3() {
        let input_str = "11111111111111111111111111111101";
        let input = u32::from_str_radix(input_str, 2).unwrap();
        let expected = 31;
        assert_eq!(Solution::hammingWeight(input), expected);
    }
}
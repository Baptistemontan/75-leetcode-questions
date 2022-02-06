pub struct Solution;

impl Solution {
    #![allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        (0..u32::BITS).map(|i| 1 << i)
            .map(|mask| n & mask)
            .fold(0, |acc, i| if i != 0 {acc + 1} else {acc})
    }
}

pub fn tester(input_str: &str, expected: i32) {
    let input = u32::from_str_radix(input_str, 2).unwrap();
    assert_eq!(Solution::hammingWeight(input), expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        tester("00000000000000000000000000001011", 3);
    }

    #[test]
    fn test2() {
        tester("00000000000000000000000010000000", 1);
    }

    #[test]
    fn test3() {
        tester("11111111111111111111111111111101", 31);
    }
}
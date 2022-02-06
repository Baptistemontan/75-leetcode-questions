pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        // could do x.reverse_bits() but not fun
        (0..u32::BITS)
            .map(|i| (x >> i) & 1)
            .fold(0, |acc, n| (acc << 1) | n)
    }
}

pub fn tester(input_str: &str, expected: u32) {
    let input = u32::from_str_radix(input_str, 2).unwrap();
    assert_eq!(Solution::reverse_bits(input), expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        tester("00000010100101000001111010011100", 964176192);
    }
}
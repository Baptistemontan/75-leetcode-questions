pub struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut carry = 0;
        let mut result = 0;
        for i in 0..(i32::BITS) {
            let mask = 1 << i;
            if (a & mask != 0) && (b & mask != 0) {
                result |= carry << i;
                carry = 1;
            } else if (a & mask != 0) || (b & mask != 0) {
                if carry == 0 {
                    result |= mask;
                }
            } else if carry != 0 {
                result |= mask;
                carry = 0;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (a, b) = (1, 2);
        let expected = a + b;
        assert_eq!(Solution::get_sum(a, b), expected);
    }

    #[test]
    fn test2() {
        let (a, b) = (2, 3);
        let expected = a + b;
        assert_eq!(Solution::get_sum(a, b), expected);
    }

    #[test]
    fn test3() {
        let (a, b) = (20, 30);
        let expected = a + b;
        assert_eq!(Solution::get_sum(a, b), expected);
    }

    #[test]
    fn test4() {
        let (a, b) = (-1, 1);
        let expected = a + b;
        assert_eq!(Solution::get_sum(a, b), expected);
    }
}


pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // looked like a fibonnaci sequence, as cs(n) = cs(n - 1) + cs(n - 2), but with different starting value
        // fibonnaci would be on range 1..n, 
        // here start a 0 to shift the values to be inlined with the starting value of the problem
        (0..n).fold((1, 0), |(curr, prev), _| (curr + prev, curr)).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
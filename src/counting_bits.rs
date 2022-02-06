pub struct Solution;

impl Solution {  
    pub fn count_bits(n: i32) -> Vec<i32> {
        if n < 2 {
            return (0..=n).collect();
        }
        let mut result: Vec<i32> = vec![0, 1];
        for i in 1..i32::BITS {
            let mask: usize = 1 << i;
            if mask > n as usize {
                break;
            }
            for j in 0..mask {
                result.push(result[j] + 1);
            }
        }
        result.truncate((n + 1) as usize);
        result
    }
}

pub fn tester(input: i32, expected: Vec<i32>) {
    let result = Solution::count_bits(input);
    assert_eq!(result, expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        tester(2, vec![0, 1, 1]);
    }

    #[test]
    fn test2() {
        tester(5, vec![0,1,1,2,1,2]);
    }

    #[test]
    fn test3() {
        tester(0, vec![0]);
    }

}
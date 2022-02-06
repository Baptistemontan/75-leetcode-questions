pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 1;
        let mut max_profit = 0;
        while right < prices.len() {
            if prices[left] < prices[right] {
                let profit = prices[right] - prices[left];
                max_profit = max_profit.max(profit);
            } else {
                left = right
            }
            right += 1;
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let prices = vec![7,1,5,3,6,4];
        let expected = 5;
        assert_eq!(Solution::max_profit(prices), expected);
    }
}
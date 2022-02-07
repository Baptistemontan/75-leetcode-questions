pub struct Solution;

impl Solution {

    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..=amount {
            for coin in coins.iter() {
                if coin <= &i {
                    dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
                }
            }
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let coins = vec![1,2,5];
        let amount = 11;
        assert_eq!(Solution::coin_change(coins, amount), 3);
    }

    #[test]
    fn test2() {
        let coins = vec![2];
        let amount = 3;
        assert_eq!(Solution::coin_change(coins, amount), -1);
    }

    #[test]
    fn test3() {
        let coins = vec![1];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0);
    }

    #[test]
    fn test4() {
        let coins = vec![2,5,10,1];
        let amount = 27;
        assert_eq!(Solution::coin_change(coins, amount), 4);
    }

    #[test]
    fn test5() {
        let coins = vec![186,419,83,408];
        let amount = 6249;
        assert_eq!(Solution::coin_change(coins, amount), 20);
    }
}
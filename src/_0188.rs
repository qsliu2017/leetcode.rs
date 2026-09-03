#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;

        // ops: [buy_1, sell_1, buy_2, sell_2, ..., buy_k, sell_k]
        // dp[i] = max profit after i ops
        let mut dp = vec![i32::MIN; 2 * k];
        for p in prices {
            for i in 0..2 * k {
                dp[i] = dp[i].max(
                    (i > 0)
                        .then(|| dp[i - 1])
                        .unwrap_or(0)
                        .saturating_add(p * if i % 2 == 0 { -1 } else { 1 }),
                );
            }
        }
        dp[2 * k - 1]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(2, vec![2, 4, 1], 2), (2, vec![3, 2, 6, 5, 0, 3], 7)];
        for (k, prices, expected) in tt {
            let output = super::Solution::max_profit(k, prices);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy_1, mut sell_1, mut buy_2, mut sell_2) =
            (i32::MIN, i32::MIN, i32::MIN, i32::MIN);
        for p in prices {
            buy_1 = buy_1.max(-p);
            sell_1 = sell_1.max(buy_1.saturating_add(p));
            buy_2 = buy_2.max(sell_1.saturating_sub(p));
            sell_2 = sell_2.max(buy_2.saturating_add(p));
        }
        sell_2
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![3, 3, 5, 0, 0, 3, 1, 4], 6),
            (vec![1, 2, 3, 4, 5], 4),
            (vec![7, 6, 4, 3, 1], 0),
            (vec![1], 0),
        ];
        for (prices, expected) in tt {
            let output = super::Solution::max_profit(prices);
            assert_eq!(expected, output);
        }
    }
}

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .into_iter()
            .rev()
            .fold((0, 0), |(max_profit, max_price), price| {
                (max_profit.max(max_price - price), max_price.max(price))
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use crate::_0121::Solution;

    #[test]
    fn test() {
        let tt = [(vec![7, 1, 5, 3, 6, 4], 5), (vec![7, 6, 4, 3, 1], 0)];
        for (input, output) in tt {
            assert_eq!(Solution::max_profit(input), output);
        }
    }
}

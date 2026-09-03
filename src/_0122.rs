#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .windows(2)
            .map(|w| {
                let [a, b] = w else {
                    unreachable!();
                };
                (b - a).max(0)
            })
            .sum()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![7, 1, 5, 3, 6, 4], 7),
            (vec![1, 2, 3, 4, 5], 4),
            (vec![7, 6, 4, 3, 1], 0),
        ];
        for (prices, expected) in tt {
            let output = super::Solution::max_profit(prices);
            assert_eq!(expected, output);
        }
    }
}

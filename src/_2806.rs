#[allow(unused)]
struct Solution;
impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (purchase_amount + 5) / 10 * 10
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(9, 90), (15, 80)];
        for (input, expected) in tt {
            let output = super::Solution::account_balance_after_purchase(input);
            assert_eq!(expected, output);
        }
    }
}

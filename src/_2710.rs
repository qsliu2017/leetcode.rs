#[allow(unused)]
struct Solution;
impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let last = num
            .as_bytes()
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(_, &b)| b != b'0')
            .map(|(i, _)| i)
            .next()
            .unwrap_or(0);
        let mut num = num;
        num.truncate(last + 1);
        num
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("51230100", "512301")];
        for (num, expected) in tt {
            let output = super::Solution::remove_trailing_zeros(num.to_string());
            assert_eq!(expected, output);
        }
    }
}

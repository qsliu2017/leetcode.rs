struct Solution;
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        s.bytes()
            .into_iter()
            .enumerate()
            .fold((0, 0), |(acc, j), (i, b)| {
                if b == b'0' {
                    (acc + i as i64 - j, j + 1)
                } else {
                    (acc, j)
                }
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("101", 1), ("100", 2), ("0111", 0)];
        for (input, expected) in tt {
            let output = super::Solution::minimum_steps(input.to_string());
            assert_eq!(expected, output);
        }
    }
}

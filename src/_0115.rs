#[allow(unused)]
struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        // dp[i] = # distinct matches of t[..i]
        let mut dp = vec![0; t.len() + 1];
        dp[0] = 1;
        for s in s.iter() {
            for (j, t) in t.iter().enumerate().rev() {
                dp[j + 1] += if s == t { dp[j] } else { 0 };
            }
        }
        dp[t.len()] as _
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("rabbbit", "rabbit", 3), ("babgbag", "bag", 5)];
        for (s, t, expected) in tt {
            let output = super::Solution::num_distinct(s.to_string(), t.to_string());
            assert_eq!(expected, output);
        }
    }
}

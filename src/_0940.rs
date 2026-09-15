#[allow(unused)]
struct Solution;
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![0; 26];
        let mut sum = 0_usize;
        for ch in s.iter().map(|&b| (b - b'a') as usize) {
            let old = dp[ch];
            let new = sum + 1;
            dp[ch] = new % 1_000_000_007;
            let diff = (new + 1_000_000_007 - old) % 1_000_000_007;
            sum = (sum + diff) % 1_000_000_007;
        }
        sum as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("abc", 7), ("aba", 6), ("aaa", 3)];
        for (s, expected) in tt {
            let output = super::Solution::distinct_subseq_ii(s.to_string());
            assert_eq!(expected, output);
        }
    }
}

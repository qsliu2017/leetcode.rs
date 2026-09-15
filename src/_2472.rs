#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        // dp[i] = max_palindromes in s[..i]
        let mut dp = vec![0; s.len() + 1];
        // is_palindrome[i][j] = true if s[i..j] is palindrome
        let mut is_palindrome = vec![vec![false; s.len() + 1]; s.len() + 1];
        for (i, &b) in s.iter().enumerate() {
            is_palindrome[i][i] = true;
            is_palindrome[i][i + 1] = true;
            let mut max_p = dp[i] + if k == 1 { 1 } else { 0 };
            for start in (0..i).rev() {
                if s[start] == b && (start + 1 > i || is_palindrome[start + 1][i]) {
                    is_palindrome[start][i + 1] = true;
                    max_p = max_p.max(dp[start] + if i + 1 >= start + k { 1 } else { 0 });
                }
            }
            dp[i + 1] = max_p;
        }
        dp[s.len()] as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("abaccdbbd", 3, 2), ("adbcda", 2, 0)];
        for (s, k, expected) in tt {
            let output = super::Solution::max_palindromes(s.to_string(), k);
            assert_eq!(expected, output);
        }
    }
}

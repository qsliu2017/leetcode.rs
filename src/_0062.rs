struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![1; n as usize]; m as usize];
        (1..m).for_each(|i| {
            (0..n).for_each(|j| {
                dp[i][j] =
                    if i > 0 { dp[i - 1][j] } else { 0 } + if j > 0 { dp[i][j - 1] } else { 0 };
            });
        });
        dp[m - 1][n - 1]
    }
}
#[cfg(test)]
mod tests {
    use crate::_0062::Solution;

    #[test]
    fn test() {
        let tt = [(3, 7, 28), (3, 2, 3), (10, 10, 48620)];
        for (m, n, output) in tt {
            assert_eq!(Solution::unique_paths(m, n), output);
        }
    }
}

struct Solution;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        // dp[i][j] is the min distance between word1[..i] and word2[..j]
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp.iter_mut().enumerate().for_each(|(i, row)| row[0] = i);
        dp[0].iter_mut().enumerate().for_each(|(j, n)| *n = j);
        (1..=m).into_iter().fold(dp, |dp, i| {
            (1..=n).into_iter().fold(dp, |mut dp, j| {
                dp[i][j] = if word1[i - 1] == word2[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1])
                };
                dp
            })
        })[m][n] as i32
    }
}
#[cfg(test)]
mod tests {
    use crate::_0072::Solution;

    #[test]
    fn test() {
        let tt = [("horse", "ros", 3), ("intention", "execution", 5)];
        for (word1, word2, output) in tt {
            assert_eq!(
                Solution::min_distance(word1.to_string(), word2.to_string()),
                output
            );
        }
    }
}

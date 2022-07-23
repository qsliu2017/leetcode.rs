struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        // Self::is_scramble_recursive(&s1, &s2)
        // Self::is_scramble_dp(&s1, &s2)
        let mut memory = HashMap::new();
        Self::is_scramble_memory(&s1, &s2, &mut memory)
    }

    fn is_scramble_recursive(s1: &str, s2: &str) -> bool {
        if s1.len() == 1 {
            return s1 == s2;
        }
        (1..s1.len()).any(|idx| {
            (Self::is_scramble_recursive(&s1[..idx], &s2[..idx])
                && Self::is_scramble_recursive(&s1[idx..], &s2[idx..]))
                || (Self::is_scramble_recursive(&s1[..idx], &s2[s2.len() - idx..])
                    && Self::is_scramble_recursive(&s1[idx..], &s2[..s2.len() - idx]))
        })
    }

    fn is_scramble_memory(s1: &str, s2: &str, memory: &mut HashMap<String, bool>) -> bool {
        let n = s1.len();
        if n == 1 {
            return s1 == s2;
        }
        if let Some(mem) = memory.get(&(s1.to_string() + s2)) {
            return *mem;
        }
        let is_scramble = (1..n).any(|k| {
            (Self::is_scramble_memory(&s1[..k], &s2[..k], memory)
                && Self::is_scramble_memory(&s1[k..], &s2[k..], memory))
                || (Self::is_scramble_memory(&s1[..k], &s2[n - k..], memory)
                    && Self::is_scramble_memory(&s1[k..], &s2[..n - k], memory))
        });
        memory.insert(s1.to_string() + s2, is_scramble);
        is_scramble
    }

    fn is_scramble_dp(s1: &str, s2: &str) -> bool {
        let n = s1.len();
        // dp[i][m][k] = s1[i..i+k] is scramble string of s2[m..m+k]
        let mut dp = vec![vec![vec![false; n + 1]; n]; n];
        for i in 0..n {
            for m in 0..n {
                dp[i][m][1] = &s1[i..i + 1] == &s2[m..m + 1];
            }
        }

        for k in 2..n + 1 {
            for i in 0..n - k + 1 {
                for m in 0..n - k + 1 {
                    dp[i][m][k] = (1..k).any(|l| {
                        (dp[i][m][l] && dp[i + l][m + l][k - l])
                            || (dp[i][m + k - l][l] && dp[i + l][m][k - l])
                    })
                }
            }
        }
        dp[0][0][n]
    }
}
#[cfg(test)]
mod tests {
    use crate::_0087::Solution;

    #[test]
    fn test() {
        let tt = [(("abcde", "caebd"), false), (("a", "a"), true)];
        for ((s1, s2), output) in tt {
            assert_eq!(
                Solution::is_scramble(s1.to_string(), s2.to_string()),
                output
            );
        }
    }
}

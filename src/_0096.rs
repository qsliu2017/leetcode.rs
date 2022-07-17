struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        Self::table[n as usize]
    }

    /// table is computed by `Self::n_table(19)`
    const table: [i32; 20] = [
        1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845,
        35357670, 129644790, 477638700, 1767263190,
    ];

    pub fn n_table(n: usize) -> Vec<i32> {
        let mut dp = vec![1; n + 1];
        for idx in 1..n + 1 {
            dp[idx] = (0..idx).fold(0, |acc, i| dp[i] * dp[idx - 1 - i] + acc)
        }
        dp
    }
}

#[cfg(test)]
mod tests {
    use crate::_0096::Solution;

    #[test]
    fn print_n_table() {
        dbg!(Solution::n_table(19));
    }

    #[test]
    fn test() {
        let tt = [(3, 5), (2, 2), (1, 1), (4, 14)];
        for (input, output) in tt {
            debug_assert_eq!(Solution::num_trees(input), output);
        }
    }
}

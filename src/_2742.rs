#[allow(unused)]
struct Solution;
impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![i32::MAX / 2; n + 1];
        dp[0] = 0;
        (0..n).fold(dp, |mut dp, i| {
            for j in (0..=n).rev() {
                dp[j] = dp[j].min(
                    dp[(j > time[i] as usize)
                        .then(|| j - time[i] as usize - 1)
                        .unwrap_or(0)]
                        + cost[i],
                );
            }
            dp
        })[n]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 2, 3, 2], vec![1, 2, 3, 2], 3),
            (vec![2, 3, 4, 2], vec![1, 1, 1, 1], 4),
            (vec![2, 2], vec![5, 5], 2),
        ];
        for (cost, time, expected) in tt {
            let output = super::Solution::paint_walls(cost, time);
            assert_eq!(expected, output);
        }
    }
}

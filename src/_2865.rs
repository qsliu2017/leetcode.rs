#[allow(unused)]
struct Solution;
impl Solution {
    pub fn maximum_sum_of_heights(heights: Vec<i32>) -> i64 {
        let n = heights.len();
        // dp[l][start] = min(heights[start..=start+l])
        // dp[l][start] = min(dp[l-1][start], heights[start+l])
        let dp = (1..n).into_iter().fold(vec![heights.clone()], |mut dp, l| {
            dp.push(
                (0..n - l)
                    .into_iter()
                    .map(|start| heights[start + l].min(dp[l - 1][start]))
                    .collect(),
            );
            dp
        });
        (0..n)
            .into_iter()
            .map(|mid| {
                (0..mid)
                    .into_iter()
                    .map(|i| heights[i].min(dp[mid - i - 1][i + 1]) as i64)
                    .sum::<i64>()
                    + heights[mid] as i64
                    + (mid + 1..n)
                        .into_iter()
                        .map(|j| heights[j].min(dp[j - mid - 1][mid]) as i64)
                        .sum::<i64>()
            })
            .max()
            .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![5, 3, 4, 1, 1], 13),
            (vec![6, 5, 3, 9, 2, 7], 22),
            (vec![3, 2, 5, 5, 2, 3], 18),
        ];
        for (input, expected) in tt {
            let output = super::Solution::maximum_sum_of_heights(input);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        // dp[run][left] = maximum [left..left+run]
        let (_, dp) = (1..n).fold(
            (nums.clone(), vec![nums.clone()]),
            |(xor_acc, mut dp), run| {
                let xor_acc = xor_acc.windows(2).map(|w| w[0] ^ w[1]).collect::<Vec<_>>();
                let dp_line = dp[run - 1]
                    .windows(2)
                    .zip(xor_acc.iter())
                    .map(|(w, &all_run)| all_run.max(w[0]).max(w[1]))
                    .collect();
                dp.push(dp_line);
                (xor_acc, dp)
            },
        );
        dbg!(&dp);
        queries
            .into_iter()
            .map(|query| {
                let [l, r] = query[..2] else { unreachable!() };
                let (l, r) = (l as usize, r as usize);
                dp[r - l][l]
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![2, 8, 4, 32, 16, 1],
                vec![vec![0, 2], vec![1, 4], vec![0, 5]],
                vec![12, 60, 60],
            ),
            (
                vec![0, 7, 3, 2, 8, 5, 1],
                vec![vec![0, 3], vec![1, 5], vec![2, 4], vec![2, 6], vec![5, 6]],
                vec![7, 14, 11, 14, 5],
            ),
        ];
        for (nums, queries, expected) in tt {
            let output = super::Solution::maximum_subarray_xor(nums, queries);
            assert_eq!(expected, output);
        }
    }
}

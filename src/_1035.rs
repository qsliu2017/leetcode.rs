#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        nums1
            .into_iter()
            .enumerate()
            .fold(vec![vec![0; m]; n], |dp, (i, n1)| {
                nums2.iter().enumerate().fold(dp, |mut dp, (j, &n2)| {
                    dp[i][j] = (if i > 0 && j > 0 { dp[i - 1][j - 1] } else { 0 }
                        + if n1 == n2 { 1 } else { 0 })
                    .max(if i > 0 { dp[i - 1][j] } else { 0 })
                    .max(if j > 0 { dp[i][j - 1] } else { 0 });
                    dp
                })
            })[n - 1][m - 1]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 4, 2], vec![1, 2, 4], 2),
            (vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2], 3),
            (vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1], 2),
        ];
        for (nums1, nums2, expected) in tt {
            let output = super::Solution::max_uncrossed_lines(nums1, nums2);
            assert_eq!(expected, output);
        }
    }
}

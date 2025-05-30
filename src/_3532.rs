#[allow(unused)]
struct Solution;
impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut root = Vec::from_iter(0..n);
        nums.windows(2)
            .enumerate()
            .filter(|(_, w)| w[1] - w[0] <= max_diff)
            .for_each(|(i, _)| root[i + 1] = root[i]);
        queries
            .into_iter()
            .map(|q| root[q[0] as usize] == root[q[1] as usize])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                2,
                vec![1, 3],
                1,
                vec![vec![0, 0], vec![0, 1]],
                vec![true, false],
            ),
            (
                4,
                vec![2, 5, 6, 8],
                2,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]],
                vec![false, false, true, true],
            ),
        ];
        for (n, nums, max_diff, queries, expected) in tt {
            let output = super::Solution::path_existence_queries(n, nums, max_diff, queries);
            assert_eq!(expected, output);
        }
    }
}

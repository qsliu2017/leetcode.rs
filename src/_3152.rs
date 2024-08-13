#[allow(unused)]
struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let parent = nums
            .windows(2)
            .enumerate()
            .filter(|(_, w)| (w[0] ^ w[1]) & 1 == 1)
            .fold((0..n).collect::<Vec<_>>(), |mut parent, (i, _)| {
                parent[i + 1] = parent[i];
                parent
            });
        queries
            .into_iter()
            .map(|q| parent[q[0] as usize] == parent[q[1] as usize])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![3, 4, 1, 2, 6], vec![vec![0, 4]], vec![false]),
            (
                vec![4, 3, 1, 6],
                vec![vec![0, 2], vec![2, 3]],
                vec![false, true],
            ),
        ];
        for (nums, queries, expected) in tt {
            let output = super::Solution::is_array_special(nums, queries);
            assert_eq!(expected, output);
        }
    }
}

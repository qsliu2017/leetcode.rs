#[allow(unused)]
struct Solution;

use std::iter::once;
impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut indices = Vec::from_iter(0..n);
        indices.sort_by_key(|&i| nums[i]);
        let mut order = indices
            .windows(2)
            .enumerate()
            .filter_map(|(idx, w)| (nums[w[1]] - nums[w[0]] > limit).then_some(idx + 1))
            .chain(once(n))
            .scan(0, |start, end| {
                let w = (*start, end);
                *start = end;
                Some(w)
            })
            .flat_map(|(start, end)| {
                let mut order = Vec::from_iter(&indices[start..end]);
                order.sort();
                order
            })
            .zip(indices.iter())
            .map(|(&from, &to)| (from, to))
            .collect::<Vec<_>>();
        order.sort();
        order.into_iter().map(|(_, to)| nums[to]).collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 5, 3, 9, 8], 2, vec![1, 3, 5, 8, 9]),
            (vec![1, 7, 6, 18, 2, 1], 3, vec![1, 6, 7, 18, 1, 2]),
            (vec![1, 7, 28, 19, 10], 3, vec![1, 7, 28, 19, 10]),
            (
                vec![1, 60, 34, 84, 62, 56, 39, 76, 49, 38],
                4,
                vec![1, 56, 34, 84, 60, 62, 38, 76, 49, 39],
            ),
        ];
        for (nums, limit, expected) in tt {
            let output = super::Solution::lexicographically_smallest_array(nums, limit);
            assert_eq!(expected, output);
        }
    }
}

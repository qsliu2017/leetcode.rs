struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = nums
            .iter()
            .fold(HashMap::new(), |mut map, &i| {
                if let Some(cnt) = map.get_mut(&i) {
                    *cnt += 1;
                } else {
                    map.insert(i, 1);
                }
                map
            })
            .into_iter()
            .collect::<Vec<_>>();
        freq.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        freq.iter().take(k as _).map(|&a| a.0).collect()
    }
}
#[cfg(test)]
mod tests {
    use crate::_0347::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
            (vec![1], 1, vec![1]),
        ];
        for (nums, k, output) in tt {
            assert_eq!(Solution::top_k_frequent(nums, k), output);
        }
    }
}

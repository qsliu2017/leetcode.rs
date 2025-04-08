use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().collect::<BTreeSet<_>>();
        if nums.range((Unbounded, Excluded(&k))).next().is_some() {
            -1
        } else {
            nums.range((Excluded(&k), Unbounded)).count() as _
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![5, 2, 5, 4, 5], 2, 2),
            (vec![2, 1, 2], 2, -1),
            (vec![9, 7, 5, 3], 1, 4),
        ];
        for (nums, k, expected) in tt {
            let output = super::Solution::min_operations(nums, k);
            assert_eq!(expected, output);
        }
    }
}

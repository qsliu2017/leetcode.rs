#[allow(unused)]
struct Solution;
use std::{collections::BTreeMap, ops::Bound::Included};
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (lower, upper) = (lower as i64, upper as i64);
        nums.into_iter()
            .map(|n| n as i64)
            .fold(
                (0, BTreeMap::new(), 0),
                |(count, mut prefix_sums, prefix_sum), num| {
                    let prefix_sum = prefix_sum + num;
                    let count = count
                        + (lower <= prefix_sum && prefix_sum <= upper)
                            .then_some(1)
                            .unwrap_or(0)
                        + prefix_sums
                            .range((
                                Included(&(prefix_sum - upper)),
                                Included(&(prefix_sum - lower)),
                            ))
                            .into_iter()
                            .map(|(_, &cnt)| cnt)
                            .sum::<i32>();
                    prefix_sums
                        .entry(prefix_sum)
                        .and_modify(|cnt| *cnt += 1)
                        .or_insert(1);
                    (count, prefix_sums, prefix_sum)
                },
            )
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![-2, 5, -1], -2, 2, 3),
            (vec![0], 0, 0, 1),
            (vec![-2147483647, 0, -2147483647, 2147483647], -564, 3864, 3),
        ];
        for (nums, lower, upper, expected) in tt {
            let output = super::Solution::count_range_sum(nums, lower, upper);
            assert_eq!(expected, output);
        }
    }
}

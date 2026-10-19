#[allow(unused)]
struct Solution;
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (start, run) = nums
            .into_iter()
            .fold((0, 0), |(start, run), n| match n.cmp(&target) {
                std::cmp::Ordering::Less => (start + 1, run),
                std::cmp::Ordering::Equal => (start, run + 1),
                std::cmp::Ordering::Greater => (start, run),
            });
        (start..start + run).collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 2, 5, 2, 3], 2, vec![1, 2]),
            (vec![1, 2, 5, 2, 3], 3, vec![3]),
            (vec![1, 2, 5, 2, 3], 5, vec![4]),
            (vec![1, 2, 5, 2, 3], 4, vec![]),
        ];
        for (nums, target, expected) in tt {
            let output = super::Solution::target_indices(nums, target);
            assert_eq!(expected, output);
        }
    }
}

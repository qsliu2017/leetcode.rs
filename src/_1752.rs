#[allow(unused)]
struct Solution;
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut rotated_iter = nums.windows(2).filter(|w| {
            let [a, b] = w else { unreachable!() };
            a > b
        });
        rotated_iter
            .next()
            .is_none_or(|_| rotated_iter.next().is_none() && nums[0] >= nums[nums.len() - 1])
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![3, 4, 5, 1, 2], true),
            (vec![2, 1, 3, 4], false),
            (vec![1, 2, 3], true),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::check(nums);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
        let (dp_m, dp_p) = nums[1..].iter().map(|&n| n as i64).fold(
            (nums[0] as i64, nums[0] as i64),
            |(dp_minus, dp_positive), num| (dp_positive - num, dp_minus.max(dp_positive) + num),
        );
        dp_m.max(dp_p)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, -2, 3, 4], 10),
            (vec![1, -1, 1, -1], 4),
            (vec![0], 0),
            (vec![1, -1], 2),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::maximum_total_cost(nums);
            assert_eq!(expected, output);
        }
    }
}

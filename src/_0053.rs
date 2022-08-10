struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((i32::MIN, 0), |(max_sum, mut last_max_sum), num| {
                last_max_sum = num + last_max_sum.max(0);
                (max_sum.max(last_max_sum), last_max_sum)
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use crate::_0053::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6),
            (vec![1], 1),
            (vec![5, 4, -1, 7, 8], 23),
        ];
        for (input, output) in tt {
            assert_eq!(Solution::max_sub_array(input), output);
        }
    }
}

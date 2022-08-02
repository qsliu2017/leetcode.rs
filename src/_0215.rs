struct Solution;
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let pos = nums.len() - k as usize;
        *nums.select_nth_unstable(pos).1
    }
}

#[cfg(test)]
mod tests {
    use crate::_0215::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![3, 2, 1, 5, 6, 4], 2, 5),
            (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4),
        ];
        for (nums, k, output) in tt {
            assert_eq!(Solution::find_kth_largest(nums, k), output);
        }
    }
}

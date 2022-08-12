struct Solution;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let mut i = 0;
        while i < n {
            let num = nums[i];
            if num == i as i32 + 1 || num > n as i32 || num <= 0 || nums[num as usize - 1] == num {
                i += 1;
                continue;
            }
            nums.swap(i, num as usize - 1);
        }
        nums.into_iter()
            .enumerate()
            .filter(|&(i, n)| i + 1 != n as usize)
            .map(|(i, _)| i + 1)
            .next()
            .unwrap_or(n + 1) as i32
    }
}
#[cfg(test)]
mod tests {
    use crate::_0041::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![1, 2, 0], 3),
            (vec![3, 4, -1, 1], 2),
            (vec![7, 8, 9, 11, 12], 1),
        ];
        for (input, output) in tt {
            assert_eq!(Solution::first_missing_positive(input), output);
        }
    }
}

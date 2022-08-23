struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<std::collections::HashSet<_>>();
        nums.iter()
            .filter(|&&num| !nums.contains(&(num - 1)))
            .fold(0, |longest, &num| {
                (1..)
                    .into_iter()
                    .filter(|i| !nums.contains(&(num + i)))
                    .next()
                    .unwrap()
                    .max(longest)
            })
    }
}
#[cfg(test)]
mod tests {
    use crate::_0128::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![100, 4, 200, 1, 3, 2], 4),
            (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
        ];
        for (input, output) in tt {
            assert_eq!(Solution::longest_consecutive(input), output);
        }
    }
}

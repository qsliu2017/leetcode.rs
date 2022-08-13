struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        nums.into_iter()
            .fold((i32::MIN, 1, 1), |(overall_max, max, min), n| {
                let (max, min) = (
                    n.max(n * if n < 0 { min } else { max }),
                    n.min(n * if n < 0 { max } else { min }),
                );
                (overall_max.max(max), max, min)
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use crate::_0152::Solution;

    #[test]
    fn test() {
        let tt = [(vec![2, 3, -2, 4], 6), (vec![-2, 0, -1], 0)];
        for (input, output) in tt {
            assert_eq!(Solution::max_product(input), output);
        }
    }
}

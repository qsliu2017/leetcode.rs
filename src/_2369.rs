#[allow(unused)]
struct Solution;
impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        nums.iter()
            .fold(
                ((false, false, true), i32::MIN, i32::MIN),
                |(is_valid, a, b), &c| {
                    (
                        (
                            is_valid.1,
                            is_valid.2,
                            (is_valid.1 && b == c)
                                || (is_valid.0
                                    && ((a == b && b == c) || (a + 1 == b && b + 1 == c))),
                        ),
                        b,
                        c,
                    )
                },
            )
            .0
             .2
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![4, 4, 4, 5, 6], true),
            (vec![1, 1, 1, 2], false),
            (
                vec![993335, 993336, 993337, 993338, 993339, 993340, 993341],
                false,
            ),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::valid_partition(nums);
            assert_eq!(expected, output);
        }
    }
}

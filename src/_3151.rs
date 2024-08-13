#[allow(unused)]
struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|w| (w[0] ^ w[1]) & 1 == 1)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1], true),
            (vec![2, 1, 4], true),
            (vec![4, 3, 1, 6], false),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::is_array_special(nums);
            assert_eq!(expected, output);
        }
    }
}

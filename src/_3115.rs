#[allow(unused)]
struct Solution;
static IS_PRIME: [bool; 100] = [
    false, true, true, false, true, false, true, false, false, false, true, false, true, false,
    false, false, true, false, true, false, false, false, true, false, false, false, false, false,
    true, false, true, false, false, false, false, false, true, false, false, false, true, false,
    true, false, false, false, true, false, false, false, false, false, true, false, false, false,
    false, false, true, false, true, false, false, false, false, false, true, false, false, false,
    true, false, true, false, false, false, false, false, true, false, false, false, true, false,
    false, false, false, false, true, false, false, false, false, false, false, false, true, false,
    false, false,
];
impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let (l, _) = nums
            .iter()
            .enumerate()
            .filter(|&(_, &n)| IS_PRIME[n as usize - 1])
            .next()
            .unwrap();
        let (r, _) = nums
            .iter()
            .enumerate()
            .rev()
            .filter(|&(_, &n)| IS_PRIME[n as usize - 1])
            .next()
            .unwrap();
        (r - l) as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![4, 2, 9, 5, 3], 3),
            (vec![4, 8, 2, 8], 0),
            (vec![1, 7], 0),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::maximum_prime_difference(nums);
            assert_eq!(expected, output);
        }
    }
}

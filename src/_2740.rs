#[allow(unused)]
struct Solution;
impl Solution {
    pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.windows(2).map(|w| w[1] - w[0]).min().unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![1, 3, 2, 4], 1), (vec![100, 1, 10], 9)];
        for (nums, expected) in tt {
            let output = super::Solution::find_value_of_partition(nums);
            assert_eq!(expected, output);
        }
    }
}

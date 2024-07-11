#[allow(unused)]
struct Solution;
impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
        let i = nums.windows(2).take_while(|w| w[0] < w[1]).count() + 1;
        if i == nums.len() {
            return (i * (i + 1) / 2) as _;
        }
        let mut nums = nums;
        nums.push(i32::MAX);
        nums.windows(2)
            .rev()
            .take_while(|w| w[0] < w[1])
            .map(|w| w[0])
            .fold((i + 1, i), |(acc, mut i), n| {
                while i > 0 && nums[i - 1] >= n {
                    i -= 1;
                }
                (acc + i + 1, i)
            })
            .0 as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 2, 3, 4], 10),
            (vec![6, 5, 7, 8], 7),
            (vec![8, 7, 6, 6], 3),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::incremovable_subarray_count(nums);
            assert_eq!(expected, output);
        }
    }
}

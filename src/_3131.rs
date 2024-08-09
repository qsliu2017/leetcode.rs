#[allow(unused)]
struct Solution;
impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        nums2.into_iter().max().unwrap() - nums1.into_iter().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![2, 6, 4], vec![9, 7, 5], 3),
            (vec![10], vec![5], -5),
            (vec![1, 1, 1, 1], vec![1, 1, 1, 1], 0),
        ];
        for (nums1, nums2, expected) in tt {
            let output = super::Solution::added_integer(nums1, nums2);
            assert_eq!(expected, output);
        }
    }
}

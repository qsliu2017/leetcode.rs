#[allow(unused)]
struct Solution;
impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        nums.chunks_mut(2)
            .for_each(|chunk| (chunk[0], chunk[1]) = (chunk[1], chunk[0]));
        nums
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![5, 4, 2, 3], vec![3, 2, 5, 4]),
            (vec![2, 5], vec![5, 2]),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::number_game(nums);
            assert_eq!(expected, output);
        }
    }
}

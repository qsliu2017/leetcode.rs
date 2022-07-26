struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut k = 2;
        for i in 2..nums.len() {
            if nums[k - 2] < nums[i] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![1, 1, 1, 2, 2, 3], 5, vec![1, 1, 2, 2, 3]),
            (
                vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
                7,
                vec![0, 0, 1, 1, 2, 3, 3],
            ),
        ];
        for (mut nums, k, output) in tt {
            assert_eq!(Solution::remove_duplicates(&mut nums), k);
            for i in 0..k as usize {
                assert_eq!(nums[i], output[i]);
            }
        }
    }
}

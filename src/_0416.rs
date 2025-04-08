#[allow(unused)]
struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;
        let mut partial_sum = vec![false; target as usize + 1];
        partial_sum[0] = true;
        nums.iter()
            .find(|&&n| {
                if n > target {
                    return false;
                }
                (n..target + 1)
                    .rev()
                    .map(|i| i as usize)
                    .for_each(|i| partial_sum[i] |= partial_sum[i - n as usize]);
                partial_sum[target as usize]
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![1, 5, 11, 5], true), (vec![1, 2, 3, 5], false)];
        for (nums, expected) in tt {
            let output = super::Solution::can_partition(nums);
            assert_eq!(expected, output);
        }
    }
}

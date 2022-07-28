struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut next_zero, mut last_two) = (0, nums.len());
        let mut ptr = 0;
        while ptr < last_two {
            let n = nums[ptr];
            if n == 0 {
                nums[ptr] = nums[next_zero];
                nums[next_zero] = 0;
                next_zero += 1;
                ptr += 1;
            } else if n == 1 {
                ptr += 1;
            } else if n == 2 {
                last_two -= 1;
                nums[ptr] = nums[last_two];
                nums[last_two] = 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::_0075::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![2, 0, 2, 1, 1, 0], vec![0, 0, 1, 1, 2, 2]),
            (vec![2, 0, 1], vec![0, 1, 2]),
        ];
        for (mut input, output) in tt {
            Solution::sort_colors(&mut input);
            assert_eq!(input, output);
        }
    }
}

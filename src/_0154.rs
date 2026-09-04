#[allow(unused)]
struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        fn find_min(nums: &[i32]) -> i32 {
            let n = nums.len();
            match n {
                0 => unreachable!(),
                1 => nums[0],
                _ => {
                    let i = n / 2;
                    let left_min = if nums[0] < nums[i] {
                        nums[0]
                    } else {
                        find_min(&nums[..i])
                    };
                    let right_min = if nums[i] < nums[n - 1] {
                        nums[i]
                    } else {
                        find_min(&nums[i..])
                    };
                    i32::min(left_min, right_min)
                }
            }
        }

        find_min(&nums)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![1, 3, 5], 1), (vec![2, 2, 2, 0, 1], 0)];
        for (nums, expected) in tt {
            let output = super::Solution::find_min(nums);
            assert_eq!(expected, output);
        }
    }
}

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        Self::rotated_search(&nums, target, 0, nums.len())
    }
    fn rotated_search(nums: &Vec<i32>, target: i32, from: usize, to: usize) -> bool {
        let n = to - from;
        if n == 0 {
            return false;
        }
        if n == 1 {
            return nums[from] == target;
        }

        let mid = (from + to) / 2;
        if nums[mid] == target {
            return true;
        }
        if nums[from] > nums[mid] {
            Self::rotated_search(nums, target, from, mid)
                || Self::ordered_search(nums, target, mid + 1, to)
        } else if nums[mid] > nums[to - 1] {
            Self::ordered_search(nums, target, from, mid)
                || Self::rotated_search(nums, target, mid + 1, to)
        } else {
            Self::rotated_search(nums, target, from, mid)
                || Self::rotated_search(nums, target, mid + 1, to)
        }
    }
    fn ordered_search(nums: &Vec<i32>, target: i32, from: usize, to: usize) -> bool {
        let n = to - from;
        if n == 0 {
            return false;
        }
        if n == 1 {
            return nums[from] == target;
        }

        let mid = (from + to) / 2;
        if nums[mid] == target {
            return true;
        }
        if nums[mid] > target {
            Self::ordered_search(nums, target, from, mid)
        } else {
            Self::ordered_search(nums, target, mid + 1, to)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::_0081::Solution;

    #[test]
    fn test() {
        let tt = [
            ((vec![2, 5, 6, 0, 0, 1, 2], 0), true),
            ((vec![2, 5, 6, 0, 0, 1, 2], 3), false),
        ];
        for ((nums, target), output) in tt {
            assert_eq!(Solution::search(nums, target), output);
        }
    }
}

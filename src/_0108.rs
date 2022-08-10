struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::vec_range_to_bst(&nums, 0, nums.len())
    }

    fn vec_range_to_bst(
        nums: &Vec<i32>,
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start < end {
            let mid = (start + end) / 2;
            let (left, right) = (
                Self::vec_range_to_bst(nums, start, mid),
                Self::vec_range_to_bst(nums, mid + 1, end),
            );
            let node = TreeNode {
                val: nums[mid],
                left,
                right,
            };
            Some(Rc::from(RefCell::from(node)))
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

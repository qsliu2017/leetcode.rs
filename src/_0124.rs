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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_path_sum = i32::MIN;
        Self::max_path_sum_helper(&root, &mut max_path_sum);
        max_path_sum
    }

    fn max_path_sum_helper(root: &Option<Rc<RefCell<TreeNode>>>, max_path_sum: &mut i32) -> i32 {
        root.as_ref()
            .and_then(|node| {
                let (left_v, right_v) = (
                    Self::max_path_sum_helper(&node.as_ref().borrow().left, max_path_sum),
                    Self::max_path_sum_helper(&node.as_ref().borrow().right, max_path_sum),
                );
                let current_v = node.as_ref().borrow().val + 0.max(left_v).max(right_v);
                *max_path_sum = (node.as_ref().borrow().val + left_v.max(0) + right_v.max(0))
                    .max(*max_path_sum);
                Some(current_v)
            })
            .unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

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

struct Solution;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev_val = None;
        return Self::inorder_visit_recursive(root.unwrap(), &mut prev_val);
    }

    fn inorder_visit_recursive(root: Rc<RefCell<TreeNode>>, prev_val: &mut Option<i32>) -> bool {
        if let Some(left) = root.borrow().left.clone() {
            if !Self::inorder_visit_recursive(left, prev_val) {
                return false;
            }
        }
        if let Some(prev_val) = prev_val {
            if !(*prev_val < root.borrow().val) {
                return false;
            }
        }
        *prev_val = Some(root.borrow().val);
        if let Some(right) = root.borrow().right.clone() {
            return Self::inorder_visit_recursive(right, prev_val);
        }
        return true;
    }

    fn inorder_visit_loop(root: Rc<RefCell<TreeNode>>) -> bool {
        let mut prev_val: Option<i32> = None;
        let mut stack = vec![];
        let mut next = Some(root);
        while !stack.is_empty() || next.is_some() {
            while let Some(n) = next {
                stack.push(n.clone());
                next = n.borrow().left.clone();
            }
            if let Some(current) = stack.pop() {
                if let Some(prev_val) = prev_val {
                    if !(prev_val < current.borrow().val) {
                        return false;
                    }
                }
                prev_val = Some(current.borrow().val);
                next = current.borrow().right.clone();
            }
        }
        return true;
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

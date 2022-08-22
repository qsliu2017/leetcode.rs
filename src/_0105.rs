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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_to_idx = inorder.iter().enumerate().map(|(i, n)| (*n, i)).collect();
        Self::build_tree_helper(
            &preorder,
            0,
            preorder.len(),
            &inorder,
            &inorder_to_idx,
            0,
            inorder.len(),
        )
    }

    fn build_tree_helper(
        preorder: &Vec<i32>,
        p_from: usize,
        p_to: usize,
        inorder: &Vec<i32>,
        inorder_to_idx: &std::collections::HashMap<i32, usize>,
        i_from: usize,
        i_to: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if p_from < p_to {
            let val = preorder[p_from];
            let &i_mid = inorder_to_idx.get(&val).unwrap();
            let (left, right) = (
                Self::build_tree_helper(
                    preorder,
                    p_from + 1,
                    p_from + i_mid - i_from + 1,
                    inorder,
                    inorder_to_idx,
                    i_from,
                    i_mid,
                ),
                Self::build_tree_helper(
                    preorder,
                    p_to - i_to + i_mid + 1,
                    p_to,
                    inorder,
                    inorder_to_idx,
                    i_mid + 1,
                    i_to,
                ),
            );
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
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

struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let n = n as usize;
        // `subtrees[i][j]` is all the subtrees of [i, j).
        // specially, `subtrees[i][i]` is `vec![None]`
        let mut subtrees: Vec<Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>> =
            vec![vec![vec![None]; n + 1]; n + 1];
        for length in 1..n + 1 {
            for i in 0..n + 1 {
                for j in i + length..n + 1 {
                    let mut range_subtrees = vec![];
                    for root in i..j {
                        for left in &subtrees[i][root] {
                            for right in &subtrees[root + 1][j] {
                                range_subtrees.push(Some(Rc::new(RefCell::new(TreeNode {
                                    val: (root + 1) as i32,
                                    left: left.clone(),
                                    right: right.clone(),
                                }))))
                            }
                        }
                    }
                    subtrees[i][j] = range_subtrees;
                }
            }
        }
        subtrees[0][n].clone()
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        Solution::generate_trees(3);
    }
}

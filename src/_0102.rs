#[allow(unused)]
struct Solution;
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels = vec![];
        if let Some(root) = root {
            let mut current = vec![root];
            while !current.is_empty() {
                let (level, next) =
                    current
                        .into_iter()
                        .fold((vec![], vec![]), |(mut level, mut next), node| {
                            level.push(node.borrow().val);
                            if let Some(left) = &node.borrow().left {
                                next.push(left.clone());
                            }
                            if let Some(right) = &node.borrow().right {
                                next.push(right.clone());
                            }
                            (level, next)
                        });
                levels.push(level);
                current = next;
            }
        }
        levels
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

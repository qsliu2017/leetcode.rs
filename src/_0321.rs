#[allow(unused)]
struct Solution;
use std::{
    cmp::Ordering,
    iter::{from_fn, zip},
};
type Node = (
    Option<usize>, /* left */
    Option<usize>, /* right */
    usize,         /* # of left children */
    usize,         /* # of right children */
);
fn construct_binary_tree(nums: &Vec<i32>, start: usize, end: usize, tree: &mut Vec<Node>) -> usize {
    let root = nums[start..end]
        .iter()
        .enumerate()
        .max_by(|(i, m), (j, n)| m.cmp(n).then(j.cmp(i)))
        .unwrap()
        .0
        + start;
    tree[root] = (
        (start < root).then(|| construct_binary_tree(nums, start, root, tree)),
        (root + 1 < end).then(|| construct_binary_tree(nums, root + 1, end, tree)),
        root - start,
        end - root - 1,
    );
    root
}
fn max_with_k_digits(
    nums: &Vec<i32>,
    tree: &Vec<Node>,
    root: usize,
    k: usize,
    slot: &mut Vec<i32>,
    base: usize,
) {
    if k == 0 {
        return;
    }
    let (left, right, n_left, n_right) = tree[root];
    let n_right = (k - 1).min(n_right);
    let n_left = k - 1 - n_right;
    slot[base + n_left] = nums[root];
    if n_left > 0 {
        max_with_k_digits(nums, tree, left.unwrap(), n_left, slot, base);
    }
    if n_right > 0 {
        max_with_k_digits(nums, tree, right.unwrap(), n_right, slot, base + n_left + 1);
    }
}
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut tree1 = vec![Node::default(); nums1.len()];
        let root1 = construct_binary_tree(&nums1, 0, nums1.len(), &mut tree1);
        let mut tree2 = vec![Node::default(); nums2.len()];
        let root2 = construct_binary_tree(&nums2, 0, nums2.len(), &mut tree2);
        (k - nums2.len().min(k)..=nums1.len().min(k))
            .map(|m| {
                let mut left = vec![0; m];
                let mut right = vec![0; k - m];
                dbg!(m, k - m);
                max_with_k_digits(&nums1, &tree1, root1, m, &mut left, 0);
                max_with_k_digits(&nums2, &tree2, root2, k - m, &mut right, 0);
                let (mut i, mut j) = (0, 0);
                from_fn(|| match (i < m, j < k - m) {
                    (false, false) => None,
                    (false, true) => {
                        j += 1;
                        Some(right[j - 1])
                    }
                    (true, false) => {
                        i += 1;
                        Some(left[i - 1])
                    }
                    (true, true) => (0..(m - i).min(k - m - j))
                        .map(|k| left[i + k].cmp(&right[j + k]))
                        .reduce(|acc, ord| acc.then(ord))
                        .unwrap_or(Ordering::Equal)
                        .then_with(|| (m - i).cmp(&(k - m - j)))
                        .is_gt()
                        .then(|| {
                            i += 1;
                            Some(left[i - 1])
                        })
                        .unwrap_or_else(|| {
                            j += 1;
                            Some(right[j - 1])
                        }),
                })
                .into_iter()
                .collect::<Vec<_>>()
            })
            .inspect(|x| {
                dbg!(x);
            })
            .max_by(|a, b| {
                zip(a.iter(), b.iter())
                    .map(|(a, b)| a.cmp(b))
                    .reduce(|acc, ord| acc.then(ord))
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![3, 4, 6, 5],
                vec![9, 1, 2, 5, 8, 3],
                5,
                vec![9, 8, 6, 5, 3],
            ),
            (vec![6, 7], vec![6, 0, 4], 5, vec![6, 7, 6, 0, 4]),
            (vec![3, 9], vec![8, 9], 3, vec![9, 8, 9]),
            (
                vec![9, 1, 2, 5, 8, 3],
                vec![3, 4, 6, 5],
                5,
                vec![9, 8, 6, 5, 3],
            ),
            (vec![5, 5, 1], vec![4, 0, 1], 3, vec![5, 5, 4]),
        ];
        for (nums1, nums2, k, expected) in tt {
            let output = super::Solution::max_number(nums1, nums2, k);
            assert_eq!(expected, output);
        }
    }
}

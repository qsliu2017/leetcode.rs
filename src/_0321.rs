#[allow(unused)]
struct Solution;
use std::{
    cmp::Ordering,
    iter::{from_fn, zip},
};
fn construct_max_sequence(nums: &Vec<i32>, m: usize) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .fold(Vec::with_capacity(m), |mut stack, (i, &n)| {
            while stack.len() + nums.len() - i > m && stack.last().is_some_and(|&top| top < n) {
                stack.pop();
            }
            if stack.len() < m {
                stack.push(n);
            }
            stack
        })
}
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        (k - nums2.len().min(k)..=nums1.len().min(k))
            .map(|m| {
                let left = construct_max_sequence(&nums1, m);
                let right = construct_max_sequence(&nums2, k - m);
                let (mut i, mut j) = (0, 0);
                from_fn(move || match (i < m, j < k - m) {
                    (false, false) => None,
                    (false, true) => {
                        j += 1;
                        Some(right[j - 1])
                    }
                    (true, false) => {
                        i += 1;
                        Some(left[i - 1])
                    }
                    (true, true) => {
                        let (mut is_gt, mut is_lt) = (false, false);
                        let is_eq = !(0..(m - i).min(k - m - j))
                            .map(|k| left[i + k].cmp(&right[j + k]))
                            .any(|ord| {
                                is_gt = ord.is_gt();
                                is_lt = ord.is_lt();
                                is_gt || is_lt
                            });
                        (is_gt || (is_eq && m - i > k - m - j))
                            .then(|| {
                                i += 1;
                                Some(left[i - 1])
                            })
                            .unwrap_or_else(|| {
                                j += 1;
                                Some(right[j - 1])
                            })
                    }
                })
                .into_iter()
                .collect::<Vec<_>>()
            })
            .max_by(|a, b| {
                let (mut is_gt, mut is_lt) = (false, false);
                zip(a.iter(), b.iter()).map(|(a, b)| a.cmp(b)).any(|ord| {
                    is_gt = ord.is_gt();
                    is_lt = ord.is_lt();
                    is_gt || is_lt
                });
                if is_gt {
                    Ordering::Greater
                } else if is_lt {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
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

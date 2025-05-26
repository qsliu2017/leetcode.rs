#[allow(unused)]
struct Solution;

use std::collections::BinaryHeap;
#[derive(PartialEq, Eq)]
struct Pair {
    a: i32,
    b: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.a + self.b).cmp(&(other.a + other.b))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        'outer: for (i, &a) in nums1.iter().enumerate() {
            'inner: for (j, &b) in nums2.iter().enumerate() {
                let pair = Pair { a, b };
                if heap.len() == k {
                    let peek = heap.peek().unwrap();
                    if &pair < peek {
                        heap.pop();
                        heap.push(pair);
                    } else {
                        if j == 0 {
                            break 'outer;
                        } else {
                            break 'inner;
                        }
                    }
                } else {
                    heap.push(pair);
                }
            }
        }
        heap.into_sorted_vec()
            .into_iter()
            .map(|pair| vec![pair.a, pair.b])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![1, 7, 11],
                vec![2, 4, 6],
                3,
                vec![vec![1, 2], vec![1, 4], vec![1, 6]],
            ),
            (
                vec![1, 1, 2],
                vec![1, 2, 3],
                2,
                vec![vec![1, 1], vec![1, 1]],
            ),
        ];
        for (nums1, nums2, k, expected) in tt {
            let output = super::Solution::k_smallest_pairs(nums1, nums2, k);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
use std::{collections::BinaryHeap, iter::from_fn};
impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let n = possible.len();
        let (acc, mut heap) = possible.into_iter().enumerate().fold(
            (0, BinaryHeap::with_capacity(n)),
            |(acc, mut heap), (i, mode)| {
                let acc = acc + if mode == 0 { -1 } else { 1 };
                if heap
                    .peek()
                    .or(Some(&(i32::MIN, 0)))
                    .is_some_and(|(max, _)| acc > *max)
                {
                    heap.push((acc, i));
                }
                (acc, heap)
            },
        );
        from_fn(move || heap.pop())
            .filter(|&(score, i)| 2 * score > acc && i + 1 < n)
            .map(|(_, i)| i as i32 + 1)
            .last()
            .unwrap_or(-1)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 0, 1, 0], 1),
            (vec![1, 1, 1, 1, 1], 3),
            (vec![0, 0], -1),
            (vec![1, 1], -1),
            (vec![0, 1, 0], 2),
        ];
        for (possible, expected) in tt {
            let output = super::Solution::minimum_levels(possible);
            assert_eq!(expected, output);
        }
    }
}

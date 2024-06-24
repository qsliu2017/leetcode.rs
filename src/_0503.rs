#[allow(unused)]
struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let (ans, heap) = nums.iter().enumerate().fold(
            (vec![-1; nums.len()], BinaryHeap::new()),
            |(mut ans, mut heap), (i, &n)| {
                while let Some(&Reverse((_, i))) = heap.peek().filter(|Reverse((m, _))| *m < n) {
                    ans[i] = n;
                    heap.pop();
                }
                heap.push(Reverse((n, i)));
                (ans, heap)
            },
        );
        nums.into_iter()
            .fold((ans, heap), |(mut ans, mut heap), n| {
                while let Some(&Reverse((_, i))) = heap.peek().filter(|Reverse((m, _))| *m < n) {
                    ans[i] = n;
                    heap.pop();
                }
                (ans, heap)
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 2, 1], vec![2, -1, 2]),
            (vec![1, 2, 3, 4, 3], vec![2, 3, 4, -1, 4]),
        ];
        for (input, expected) in tt {
            let output = super::Solution::next_greater_elements(input);
            assert_eq!(expected, output);
        }
    }
}

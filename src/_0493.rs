#[allow(unused)]
struct Solution;
use std::collections::{BTreeSet, HashMap};
fn lsb(x: usize) -> usize {
    ((x ^ (x - 1)) + 1) >> 1
}
impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let num_to_index = nums
            .iter()
            .flat_map(|&n| [n as i64, 2 * n as i64])
            .collect::<BTreeSet<_>>()
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, n)| (n, i + 1))
            .collect::<HashMap<_, _>>();
        nums.into_iter()
            .fold(
                (0, vec![0; num_to_index.len() + 1]),
                |(mut count, mut bitree), n| {
                    let n = n as i64;
                    let mut index = num_to_index[&(n * 2)] - 1;
                    while index > 0 {
                        count += bitree[index];
                        index -= lsb(index);
                    }
                    let mut index = num_to_index[&n];
                    while index < bitree.len() {
                        bitree[index] += 1;
                        index += lsb(index);
                    }
                    (count, bitree)
                },
            )
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![1, 3, 2, 3, 1], 2), (vec![2, 4, 3, 5, 1], 3)];
        for (input, expected) in tt {
            let output = super::Solution::reverse_pairs(input);
            assert_eq!(expected, output);
        }
    }
}

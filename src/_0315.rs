#[allow(unused)]
struct Solution;
use std::collections::{BTreeSet, HashMap};
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let num_to_index = nums
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(i, &num)| (num, i + 1))
            .collect::<HashMap<_, _>>();
        let n = nums.len();
        nums.into_iter()
            .map(|num| num_to_index.get(&num).unwrap())
            .enumerate()
            .rev()
            .fold(
                (vec![0; n], vec![0; num_to_index.len() + 1]),
                |(mut res, mut bitree), (i, &index)| {
                    let mut find_index = index - 1;
                    loop {
                        res[i] += bitree[find_index];
                        if find_index == 0 {
                            break;
                        }
                        find_index -= (((find_index - 1) ^ find_index) + 1) >> 1;
                    }
                    let mut update_index = index;
                    while update_index < num_to_index.len() {
                        bitree[update_index] += 1;
                        if update_index == 0 {
                            break;
                        }
                        update_index += (((update_index - 1) ^ update_index) + 1) >> 1;
                    }
                    (res, bitree)
                },
            )
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![5, 2, 6, 1], vec![2, 1, 1, 0]),
            (vec![-1], vec![0]),
            (vec![-1, -1], vec![0, 0]),
        ];
        for (input, expected) in tt {
            let output = super::Solution::count_smaller(input);
            assert_eq!(expected, output);
        }
    }
}

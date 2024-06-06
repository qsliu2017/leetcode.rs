struct Solution;
use std::{cmp::Ordering, collections::HashMap};
impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut sorted = nums.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        let indexes = sorted
            .into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut indexes, (i, n)| {
                indexes.insert(n, i);
                indexes
            });
        let add = |bitree: &mut Vec<i32>, mut index: usize| {
            index += 1;
            while index < n {
                bitree[index] += 1;
                index += ((index ^ (index - 1)) + 1) >> 1;
            }
        };
        let get = |bitree: &Vec<i32>, mut index: usize| -> i32 {
            let mut s = 0;
            while index > 0 {
                s += bitree[index];
                index -= ((index ^ (index - 1)) + 1) >> 1;
            }
            s
        };
        let (arr1, arr2, _, _) = nums.into_iter().enumerate().fold(
            (
                Vec::with_capacity(n),
                Vec::with_capacity(n),
                vec![0; n + 1],
                vec![0; n + 1],
            ),
            |(mut arr1, mut arr2, mut sorted1, mut sorted2), (i, n)| {
                let index = *indexes.get(&n).unwrap();
                let (arr, sorted) = match i {
                    0 => (&mut arr1, &mut sorted1),
                    1 => (&mut arr2, &mut sorted2),
                    _ => match get(&sorted1, index).cmp(&get(&sorted2, index)) {
                        Ordering::Greater => (&mut arr1, &mut sorted1),
                        Ordering::Less => (&mut arr2, &mut sorted2),
                        Ordering::Equal => {
                            if arr1.len() <= arr2.len() {
                                (&mut arr1, &mut sorted1)
                            } else {
                                (&mut arr2, &mut sorted2)
                            }
                        }
                    },
                };
                arr.push(n);
                add(sorted, index);
                (arr1, arr2, sorted1, sorted2)
            },
        );
        arr1.into_iter().chain(arr2.into_iter()).collect()
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let tt = [
            (vec![2, 1, 3, 3], vec![2, 3, 1, 3]),
            (vec![5, 14, 3, 1, 2], vec![5, 3, 1, 2, 14]),
            (vec![3, 3, 3, 3], vec![3, 3, 3, 3]),
        ];
        for (input, expected) in tt {
            let output = super::Solution::result_array(input);
            assert_eq!(expected, output);
        }
    }
}

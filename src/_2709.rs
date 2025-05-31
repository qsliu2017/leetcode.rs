#[allow(unused)]
struct Solution;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let n = nums.len();
        // 317 ** 2 > 1e5
        // primes.len() == 66
        let primes = (2..=317)
            .filter(|&num| (2..num).all(|factor| num % factor != 0))
            .collect::<Vec<_>>();
        let mut union_set = Vec::from_iter(0..nums.len());
        fn find(union_set: &mut Vec<usize>, i: usize) -> usize {
            let parent = union_set[i];
            if parent == i {
                parent
            } else {
                let root = find(union_set, parent);
                union_set[i] = root;
                root
            }
        }
        let mut union = |i: usize, j: usize| {
            let x = find(&mut union_set, i);
            let y = find(&mut union_set, j);
            if x == y {
                false
            } else {
                let (x, y) = (x.min(y), x.max(y));
                union_set[y] = x;
                true
            }
        };
        let mut n_merge = 0;
        let mut prime_factor_to_root = HashMap::new();
        for (idx, mut num) in nums.into_iter().enumerate() {
            let mut factors = HashSet::new();
            for &prime in primes.iter() {
                if num < prime {
                    break;
                }
                if num % prime != 0 {
                    continue;
                }
                factors.insert(prime);
                while num % prime == 0 {
                    num /= prime;
                }
            }
            if num > 1 {
                factors.insert(num);
            }
            n_merge += factors
                .into_iter()
                .filter(|&prime_factor| {
                    if let Some(&root) = prime_factor_to_root.get(&prime_factor) {
                        union(root, idx)
                    } else {
                        prime_factor_to_root.insert(prime_factor, idx);
                        false
                    }
                })
                .count();
        }
        n - n_merge == 1
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![2, 3, 6], true),
            (vec![3, 9, 5], false),
            (vec![4, 3, 12, 8], true),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::can_traverse_all_pairs(nums);
            assert_eq!(expected, output);
        }
    }
}

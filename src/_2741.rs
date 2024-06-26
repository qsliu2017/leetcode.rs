#[allow(unused)]
struct Solution;
use std::{collections::HashMap, iter::once};
impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let connection = nums
            .iter()
            .enumerate()
            .map(|(i, n)| {
                nums.iter()
                    .enumerate()
                    .rev()
                    .map(|(j, m)| i != j && (n % m == 0 || m % n == 0))
                    .fold(0, |acc, is_connected| {
                        (acc << 1) + if is_connected { 1 } else { 0 }
                    })
            })
            .collect::<Vec<_>>();
        (1..n) /* n visited */
            .fold(
                (0..n)
                    .map(|i| HashMap::from_iter(once((1 << i, 1))))
                    .collect::<Vec<_>>(),
                |dp /* dp[last][visited bitmap] = count */, _| {
                    (0..n) /* next to visit */
                        .map(|next| {
                            dp.iter()
                                .enumerate()
                                .filter(|&(last, _)| (1 << last) & connection[next] != 0)
                                .flat_map(|(_, counts)| counts.iter())
                                .filter(|(&visited, _)| visited & (1 << next) == 0)
                                .fold(HashMap::new(), |mut dp, (visited, count)| {
                                    dp.entry(visited | 1 << next)
                                        .and_modify(|c| {
                                            *c += count;
                                            *c %= 1000000007;
                                        })
                                        .or_insert(count % 1000000007);
                                    dp
                                })
                        })
                        .collect()
                },
            )
            .into_iter()
            .flat_map(|counts| counts.into_values())
            .reduce(|acc, count| (acc + count) % 1000000007)
            .unwrap_or_default()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![2, 3, 6], 2),
            (vec![1, 4, 3], 2),
            (
                vec![
                    1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
                ],
                178290591,
            ),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::special_perm(nums);
            assert_eq!(expected, output);
        }
    }
}

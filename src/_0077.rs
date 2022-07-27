struct Solution;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Self::sub_combine(1, n + 1, k)
    }
    // choose k numbers in [from, to)
    fn sub_combine(from: i32, to: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 0 {
            return vec![vec![]];
        }
        if to - from == k {
            return vec![(from..to).collect()];
        }
        let mut sub_combine: Vec<Vec<i32>> = Self::sub_combine(from + 1, to, k - 1)
            .into_iter()
            .map(|mut c| {
                c.push(from);
                c
            })
            .collect();
        sub_combine.append(&mut Self::sub_combine(from + 1, to, k));
        sub_combine
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::_0077::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                4,
                2,
                vec![
                    vec![2, 4],
                    vec![3, 4],
                    vec![2, 3],
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                ],
            ),
            (1, 1, vec![vec![1]]),
        ];
        for (n, k, output) in tt {
            assert_eq!(Solution::combine(n, k), output);
        }
    }
}

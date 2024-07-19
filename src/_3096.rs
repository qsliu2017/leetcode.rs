#[allow(unused)]
struct Solution;
impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let n = possible.len();
        let sum = possible
            .iter()
            .map(|&mode| if mode == 0 { -1 } else { 1 })
            .sum::<i32>();
        let mut acc = 0;
        possible
            .into_iter()
            .take(n - 1)
            .enumerate()
            .filter_map(|(i, mode)| {
                acc += if mode == 0 { -1 } else { 1 };
                (acc * 2 > sum).then_some(i as i32 + 1)
            })
            .next()
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

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        (1..num_rows).fold(vec![vec![1]], |mut rows, _| {
            let row = std::iter::once(1)
                .chain(rows.last().unwrap().windows(2).map(|w| w[0] + w[1]))
                .chain(std::iter::once(1))
                .collect();
            rows.push(row);
            rows
        })
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                5,
                vec![
                    vec![1],
                    vec![1, 1],
                    vec![1, 2, 1],
                    vec![1, 3, 3, 1],
                    vec![1, 4, 6, 4, 1],
                ],
            ),
            (1, vec![vec![1]]),
        ];
        for (num_rows, expected) in tt {
            let output = super::Solution::generate(num_rows);
            assert_eq!(expected, output);
        }
    }
}

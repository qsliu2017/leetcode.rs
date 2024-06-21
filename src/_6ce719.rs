#[allow(unused)]
struct Solution;
use std::iter::zip;
impl Solution {
    pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
        zip(temperature_a.windows(2), temperature_b.windows(2))
            .fold((0, 0), |(ans, acc), (a, b)| {
                let acc = if a[0].cmp(&a[1]) == b[0].cmp(&b[1]) {
                    acc + 1
                } else {
                    0
                };
                (ans.max(acc), acc)
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![21, 18, 18, 18, 31], vec![34, 32, 16, 16, 17], 2),
            (
                vec![5, 10, 16, -6, 15, 11, 3],
                vec![16, 22, 23, 23, 25, 3, -16],
                3,
            ),
        ];
        for (temperature_a, temperature_b, expected) in tt {
            let output = super::Solution::temperature_trend(temperature_a, temperature_b);
            assert_eq!(expected, output);
        }
    }
}

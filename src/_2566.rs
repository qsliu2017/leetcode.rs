#[allow(unused)]
struct Solution;
impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let s = num.to_string();
        let (max, _, min, _) = s.bytes().into_iter().map(|b| (b - b'0') as i32).fold(
            (0, None, 0, None),
            |(max, max_digit, min, min_digit), b| {
                let (max, max_digit) = if let Some(target) = max_digit {
                    (max * 10 + if target == b { 9 } else { b }, max_digit)
                } else {
                    (max * 10 + 9, (b != 9).then_some(b))
                };
                let (min, min_digit) = if let Some(target) = min_digit {
                    (min * 10 + if target == b { 0 } else { b }, min_digit)
                } else {
                    (min * 10, (b != 0).then_some(b))
                };
                (max, max_digit, min, min_digit)
            },
        );
        max - min
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(11891, 99009), (90, 99)];
        for (num, expected) in tt {
            let output = super::Solution::min_max_difference(num);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
use std::{cmp::Ordering, iter::from_fn};
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut n = num;
        from_fn(move || {
            (n != 0).then(|| {
                let digit = n % 10;
                n /= 10;
                digit
            })
        })
        .into_iter()
        .enumerate()
        .fold(
            ((i32::MIN, 0), None),
            |(max_digit, swap_tuple), (i, d)| match d.cmp(&max_digit.0) {
                Ordering::Greater => ((d, i), swap_tuple),
                Ordering::Less => (max_digit, Some(((d, i), max_digit))),
                Ordering::Equal => (max_digit, swap_tuple),
            },
        )
        .1
        .map_or(0, |((d, i), (e, j))| {
            (e - d) * 10_i32.pow(i as _) + (d - e) * 10_i32.pow(j as _)
        }) + num
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(2736, 7236), (9973, 9973)];
        for (input, expected) in tt {
            let output = super::Solution::maximum_swap(input);
            assert_eq!(expected, output);
        }
    }
}

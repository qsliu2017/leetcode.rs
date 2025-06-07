#[allow(unused)]
struct Solution;
use std::iter::from_fn;
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut i = 0;
        from_fn(move || {
            if i == 0 {
                i = 1;
                return Some(i);
            }
            if i * 10 <= n {
                i *= 10;
                return Some(i);
            }
            while i != 0 {
                if i % 10 < 9 && i + 1 <= n {
                    i = i + 1;
                    return Some(i);
                }
                i = i / 10;
            }
            None
        })
        .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (13, vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]),
            (2, vec![1, 2]),
        ];
        for (n, expected) in tt {
            let output = super::Solution::lexical_order(n);
            assert_eq!(expected, output);
        }
    }
}

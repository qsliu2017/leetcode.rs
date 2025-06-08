use std::i32;

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let n_descendants = |cur: i32| {
            let (mut first, mut last) = (cur, cur);
            let mut acc = 0;
            while first.checked_mul(10).unwrap_or(i32::MAX) <= n {
                first = first * 10;
                last = last
                    .checked_mul(10)
                    .and_then(|n| n.checked_add(9))
                    .unwrap_or(i32::MAX)
                    .min(n);
                acc += last - first + 1;
            }
            acc
        };
        let mut i = 1;
        let mut k = k - 1;
        while k > 0 {
            let d = n_descendants(i);
            if d < k {
                k -= d;
                i += 1;
            } else {
                i *= 10;
            }
            k -= 1;
        }
        i
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(13, 2, 10), (1, 1, 1)];
        for (n, k, expected) in tt {
            let output = super::Solution::find_kth_number(n, k);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let (min_even, max_odd) = s
            .bytes()
            .into_iter()
            .fold([0; 26], |mut count, ch| {
                count[(ch - b'a') as usize] += 1;
                count
            })
            .into_iter()
            .filter(|&cnt| cnt > 0)
            .fold((i32::MAX, 0), |(min_even, max_odd), cnt| {
                if cnt % 2 == 0 {
                    (min_even.min(cnt), max_odd)
                } else {
                    (min_even, max_odd.max(cnt))
                }
            });
        max_odd - min_even
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("aaaaabbc", 3), ("abcabcab", 1)];
        for (s, expected) in tt {
            let output = super::Solution::max_difference(s.to_string());
            assert_eq!(expected, output);
        }
    }
}

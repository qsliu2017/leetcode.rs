#[allow(unused)]
struct Solution;
use std::iter;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        // dp[i] = min cut of s[..i]
        let mut min_cut = vec![usize::MAX; n + 1];
        min_cut[0] = 0;

        // for all i, [backwards[i], current_index) is a candidate
        let mut backwards: Vec<usize> = vec![];
        for (i, &ch) in s.iter().enumerate() {
            backwards = backwards
                .into_iter()
                .chain(iter::once(i)) // try match previous ch
                .filter_map(|j| (j > 0 && s[j - 1] == ch).then(|| j - 1))
                .chain(iter::once(i)) // [current_index, current_index+1)
                .collect();
            min_cut[i + 1] = backwards
                .iter()
                .map(|&i| min_cut[i] + if i > 0 { 1 } else { 0 })
                .min()
                .unwrap();
        }
        min_cut[n] as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("aab", 1), ("a", 0), ("ab", 1)];
        for (s, expected) in tt {
            let output = super::Solution::min_cut(s.to_string());
            assert_eq!(expected, output);
        }
    }
}

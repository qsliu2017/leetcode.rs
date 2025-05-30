#[allow(unused)]
struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let n = text.len() / 2;
        (1..=n)
            .flat_map(|len| {
                (0..=text.len() - 2 * len).map(move |start| (start, start + len, start + 2 * len))
            })
            .filter(|&(start, mid, end)| &text[start..mid] == &text[mid..end])
            .map(|(start, _, end)| &text[start..end])
            .collect::<HashSet<_>>()
            .len() as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("abcabcabc", 3), ("leetcodeleetcode", 2)];
        for (text, expected) in tt {
            let output = super::Solution::distinct_echo_substrings(text.to_string());
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn smallest_string(s: String) -> String {
        let mut s = s;
        let bytes = unsafe { s.as_bytes_mut() };
        if let Some((i, _)) = bytes.iter().enumerate().filter(|(_, &b)| b != b'a').next() {
            bytes[i..]
                .iter_mut()
                .take_while(|b| **b != b'a')
                .for_each(|b| *b -= 1);
        } else {
            bytes[bytes.len() - 1] = b'z';
        }
        s
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            ("cbabc", "baabc"),
            ("acbbc", "abaab"),
            ("leetcode", "kddsbncd"),
        ];
        for (s, expected) in tt {
            let output = super::Solution::smallest_string(s.to_string());
            assert_eq!(expected, output);
        }
    }
}

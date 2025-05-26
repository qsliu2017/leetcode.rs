#[allow(unused)]
struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut iter_a = a.bytes().map(|b| b - b'0').rev();
        let mut iter_b = b.bytes().map(|b| b - b'0').rev();
        let mut addon = 0;
        let mut bytes = std::iter::from_fn(move || match (iter_a.next(), iter_b.next()) {
            (Some(a), Some(b)) => {
                let sum = a + b + addon;
                addon = sum / 2;
                Some(sum % 2)
            }
            (None, Some(e)) | (Some(e), None) => {
                let sum = e + addon;
                addon = sum / 2;
                Some(sum % 2)
            }
            (None, None) => {
                if addon == 0 {
                    None
                } else {
                    addon = 0;
                    Some(1)
                }
            }
        })
        .map(|b| b + b'0')
        .collect::<Vec<_>>();
        bytes.reverse();
        String::from_utf8(bytes).unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("11", "1", "100"), ("1010", "1011", "10101")];
        for (a, b, expected) in tt {
            let output = super::Solution::add_binary(a.to_string(), b.to_string());
            assert_eq!(expected, output);
        }
    }
}

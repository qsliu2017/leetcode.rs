#[allow(unused)]
struct Solution;
impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut s = s;
        let mut it = unsafe { s.as_bytes_mut() }.iter_mut();
        let mut k = k;
        while let Some(c) = it.next() {
            let to_a = (*c - b'a').min(b'z' + 1 - *c) as i32;
            if k >= to_a {
                *c = b'a';
                k -= to_a;
            } else {
                *c -= k as u8;
                break;
            }
        }
        s
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            ("zbbz", 3, "aaaz"),
            ("xaxcd", 4, "aawcd"),
            ("lol", 0, "lol"),
        ];
        for (s, k, expected) in tt {
            let output = super::Solution::get_smallest_string(s.to_string(), k);
            assert_eq!(expected.to_string(), output);
        }
    }
}

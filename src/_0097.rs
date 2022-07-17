struct Solution;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // Self::is_interleave_recursive(s1.as_bytes(), s2.as_bytes(), s3.as_bytes())
        Self::is_interleave_dp(s1.as_bytes(), s2.as_bytes(), s3.as_bytes())
    }

    fn is_interleave_dp(a: &[u8], b: &[u8], c: &[u8]) -> bool {
        if a.len() + b.len() != c.len() {
            return false;
        }
        let mut dp = vec![false; b.len() + 1];
        for i in 0..a.len() + 1 {
            for j in 0..b.len() + 1 {
                *dp.get_mut(j).unwrap() = (i == 0 && j == 0)
                    || (i > 0 && *dp.get(j).unwrap() && a[i - 1] == c[i + j - 1])
                    || (j > 0 && *dp.get(j - 1).unwrap() && b[j - 1] == c[i + j - 1]);
            }
            if dp.iter().all(|&x| !x) {
                return false;
            }
        }
        *dp.get(b.len()).unwrap()
    }

    fn is_interleave_recursive(a: &[u8], b: &[u8], c: &[u8]) -> bool {
        if a.len() + b.len() != c.len() {
            false
        } else if a.is_empty() {
            b.eq(c)
        } else if b.is_empty() {
            a.eq(c)
        } else {
            match (a[0] == c[0], b[0] == c[0]) {
                (true, true) => {
                    Self::is_interleave_recursive(&a[1..], b, &c[1..])
                        || Self::is_interleave_recursive(a, &b[1..], &c[1..])
                }
                (true, _) => Self::is_interleave_recursive(&a[1..], b, &c[1..]),
                (_, true) => Self::is_interleave_recursive(a, &b[1..], &c[1..]),
                (_, _) => false,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::_0097::Solution;

    #[test]
    fn test() {
        let tt = [
            (("aabcc", "dbbca", "aadbbcbcac"), true),
            (("aabcc", "dbbca", "aadbbbaccc"), false),
            (("", "", ""), true),
        ];
        for ((a, b, c), except) in tt {
            debug_assert_eq!(
                Solution::is_interleave(a.to_string(), b.to_string(), c.to_string()),
                except
            );
        }
    }
}

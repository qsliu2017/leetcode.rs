#[allow(unused)]
struct Solution;
impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut buf = vec![0_u8; lcp.len()];
        for ch in b'a'..=b'z' {
            let Some(i) = buf
                .iter()
                .enumerate()
                .find_map(|(i, &b)| (b == 0).then_some(i))
            else {
                break;
            };
            buf[i] = ch;
            (i + 1..n)
                .filter(|&j| lcp[i][j] > 0)
                .for_each(|j| buf[j] = ch);
        }
        for i in 0..n {
            if buf[i] == 0 {
                return String::new();
            }
            for j in i..n {
                let r = lcp[i][j] as usize;
                if r != lcp[j][i] as _ || j + r > n || buf[i..i + r] != buf[j..j + r] {
                    return String::new();
                }
                if j + r == n {
                    continue;
                } else if j + r > n || buf[i + r] == buf[j + r] {
                    return String::new();
                }
            }
        }
        String::from_utf8(buf).unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec![4, 0, 2, 0],
                    vec![0, 3, 0, 1],
                    vec![2, 0, 2, 0],
                    vec![0, 1, 0, 1],
                ],
                "abab",
            ),
            (
                vec![
                    vec![4, 3, 2, 1],
                    vec![3, 3, 2, 1],
                    vec![2, 2, 2, 1],
                    vec![1, 1, 1, 1],
                ],
                "aaaa",
            ),
            (
                vec![
                    vec![4, 3, 2, 1],
                    vec![3, 3, 2, 1],
                    vec![2, 2, 2, 1],
                    vec![1, 1, 1, 3],
                ],
                "",
            ),
        ];
        for (lcp, expected) in tt {
            let output = super::Solution::find_the_string(lcp);
            assert_eq!(expected, output);
        }
    }
}

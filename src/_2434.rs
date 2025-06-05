#[allow(unused)]
struct Solution;
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let rev_min = s
            .bytes()
            .rev()
            .scan(b'z' + 1, |min, b| {
                let r = *min;
                *min = b.min(*min);
                Some(r)
            })
            .collect::<Vec<_>>();
        let (_, res) = s.bytes().zip(rev_min.into_iter().rev()).fold(
            (Vec::with_capacity(s.len()), Vec::with_capacity(s.len())),
            |(mut stack, mut res), (b, min)| {
                stack.push(b);
                while let Some(&last) = stack.last().filter(|&&last| last <= min) {
                    stack.pop();
                    res.push(last);
                }
                (stack, res)
            },
        );
        String::from_utf8(res).unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("zza", "azz"), ("bac", "abc"), ("bdda", "addb")];
        for (s, expected) in tt {
            let output = super::Solution::robot_with_string(s.to_string());
            assert_eq!(expected, output);
        }
    }
}

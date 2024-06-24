#[allow(unused)]
struct Solution;
use std::iter::once;
impl Solution {
    pub fn smallest_beautiful_string(s: String, k: i32) -> String {
        let k = k as u8;
        let bytes = s
            .as_bytes()
            .into_iter()
            .map(|&b| b - b'a')
            .collect::<Vec<_>>();
        let n = s.len();
        if let Some(i) = (0..n)
            .rev()
            .filter(|&i| {
                (bytes[i] + 1..k)
                    .filter(|&b| (i < 1 || b != bytes[i - 1]) && (i < 2 || b != bytes[i - 2]))
                    .next()
                    .is_some()
            })
            .next()
        {
            let updated = (bytes[i] + 1..k)
                .filter(|&b| (i < 1 || b != bytes[i - 1]) && (i < 2 || b != bytes[i - 2]))
                .next()
                .unwrap();
            String::from_utf8(
                bytes
                    .iter()
                    .copied()
                    .take(i)
                    .chain(once(updated))
                    .chain(
                        (i + 1..n)
                            .fold(
                                (
                                    Vec::with_capacity(n - i - 1),
                                    (i > 0).then(|| bytes[i - 1]),
                                    Some(updated),
                                ),
                                |(mut cont, a, b), _| {
                                    let c = (0..4)
                                        .filter(|&c| {
                                            a.map_or(true, |a| a != c) && b.map_or(true, |b| b != c)
                                        })
                                        .next();
                                    cont.push(c.unwrap());
                                    (cont, b, c)
                                },
                            )
                            .0
                            .into_iter(),
                    )
                    .map(|b| b + b'a')
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        } else {
            "".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("abcz", 26, "abda"), ("dc", 4, "")];
        for (s, k, expected) in tt {
            let output = super::Solution::smallest_beautiful_string(s.to_string(), k);
            assert_eq!(expected, output);
        }
    }
}

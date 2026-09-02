#[allow(unused)]
struct Solution;

enum Pattern<'a> {
    Star,
    Question(usize),
    Exact(&'a [u8]),
}

impl Solution {
    fn is_match_recursive(
        s: &[u8],
        i: usize,
        patterns: &Vec<Pattern<'_>>,
        p: usize,
        mem: &mut Vec<Vec<Option<bool>>>,
    ) -> bool {
        if i > s.len() {
            return false;
        }
        if p == patterns.len() {
            return i == s.len();
        }
        if let Some(matches) = mem[i][p] {
            return matches;
        }
        let pattern = &patterns[p];
        let matches = match pattern {
            Pattern::Star => {
                (i..=s.len()).any(|j| Self::is_match_recursive(s, j, patterns, p + 1, mem))
            }
            Pattern::Question(n) => Self::is_match_recursive(s, i + n, patterns, p + 1, mem),
            Pattern::Exact(items) => {
                let n = items.len();
                i + n <= s.len()
                    && &s[i..i + n] == *items
                    && Self::is_match_recursive(s, i + n, patterns, p + 1, mem)
            }
        };
        mem[i][p] = Some(matches);

        matches
    }

    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let patterns = p
            .chunk_by(|&a, &b| match a {
                b'*' | b'?' => a == b,
                _ => !matches!(b, b'*' | b'?'),
            })
            .map(|chunk| match chunk[0] {
                b'*' => Pattern::Star,
                b'?' => Pattern::Question(chunk.len()),
                _ => Pattern::Exact(chunk),
            })
            .collect::<Vec<_>>();

        // mem[i][p] = Some(true) iff. s[i..] matches group[p..]
        let mut mem = vec![vec![None; patterns.len() + 1]; s.len() + 1];

        Self::is_match_recursive(s, 0, &patterns, 0, &mut mem)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            ("aa", "a", false),
            ("aa", "*", true),
            ("cb", "?a", false),
            ("aa", "**", true),
            ("aa", "?*", true),
            ("aa", "???", false),
            ("mississippi", "m??*ss*?i*pi", false),
        ];
        for (s, p, expected) in tt {
            let output = super::Solution::is_match(s.into(), p.into());
            assert_eq!(expected, output);
        }
    }
}

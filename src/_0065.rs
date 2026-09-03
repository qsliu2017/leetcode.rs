#[allow(unused)]
struct Solution;

trait Matcher: FnOnce(&[u8]) -> Result<&[u8], ()> {
    fn optional(self) -> impl FnOnce(&[u8]) -> Result<&[u8], ()>;
    fn and_then(
        self,
        f: impl FnOnce(&[u8]) -> Result<&[u8], ()>,
    ) -> impl FnOnce(&[u8]) -> Result<&[u8], ()>;
    fn or_else(
        self,
        f: impl FnOnce(&[u8]) -> Result<&[u8], ()>,
    ) -> impl FnOnce(&[u8]) -> Result<&[u8], ()>;
}

impl<F: FnOnce(&[u8]) -> Result<&[u8], ()>> Matcher for F {
    fn optional(self) -> impl FnOnce(&[u8]) -> Result<&[u8], ()> {
        |s: &[u8]| self(s).or_else(|_| Ok(s))
    }

    fn and_then(
        self,
        f: impl FnOnce(&[u8]) -> Result<&[u8], ()>,
    ) -> impl FnOnce(&[u8]) -> Result<&[u8], ()> {
        |s: &[u8]| self(s).and_then(f)
    }

    fn or_else(
        self,
        f: impl FnOnce(&[u8]) -> Result<&[u8], ()>,
    ) -> impl FnOnce(&[u8]) -> Result<&[u8], ()> {
        |s: &[u8]| self(s).or_else(|_| f(s))
    }
}

fn match_numbers(s: &[u8]) -> Result<&[u8], ()> {
    s.iter()
        .enumerate()
        .take_while(|(_, b)| matches!(b, b'0'..=b'9'))
        .last()
        .map(|(i, _)| &s[i + 1..])
        .ok_or(())
}

fn match_sign(s: &[u8]) -> Result<&[u8], ()> {
    s.split_first()
        .filter(|(b, _)| matches!(b, b'+' | b'-'))
        .map(|(_, rest)| rest)
        .ok_or(())
}

fn match_exact(p: &'static str) -> impl FnOnce(&[u8]) -> Result<&[u8], ()> {
    move |s: &[u8]| {
        s.split_at_checked(p.len())
            .and_then(|(first, rest)| (first == p.as_bytes()).then_some(rest))
            .ok_or(())
    }
}

// numbers with as most one '.'
fn match_fraction(s: &[u8]) -> Result<&[u8], ()> {
    let mut dot = false;
    let mut i = 0;
    for b in s {
        match b {
            b'0'..=b'9' => {}
            b'.' if !dot => {
                dot = true;
            }
            _ => {
                break;
            }
        }
        i += 1;
    }
    (i > if dot { 1 } else { 0 }).then(|| &s[i..]).ok_or(())
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let match_integer = match_sign.optional().and_then(match_numbers);

        let s = s.as_bytes();

        match_sign.optional().and_then(match_fraction).and_then(
            (match_exact("e").or_else(match_exact("E")))
                .and_then(match_integer)
                .optional(),
        )(s)
        .inspect(|s| {
            dbg!(s);
        })
        .is_ok_and(|rest| rest.is_empty())
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            ("0", true),
            ("e", false),
            (".", false),
            (".1", true),
            ("-1E+3", true),
        ];
        for (s, expected) in tt {
            let output = super::Solution::is_number(s.into());
            assert_eq!(expected, output, "{}", s);
        }
    }
}

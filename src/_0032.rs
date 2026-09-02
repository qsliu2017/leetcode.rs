#[allow(unused)]
struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let (_, _, max_pair_ltr) =
            s.iter()
                .fold((0, 0, 0), |(mut left, mut right, mut max_pair), p| {
                    match p {
                        b'(' => {
                            left += 1;
                        }
                        b')' => {
                            right += 1;
                        }
                        _ => unreachable!(),
                    }
                    if right == left {
                        max_pair = max_pair.max(right);
                    }
                    if right > left {
                        (left, right) = (0, 0);
                    }
                    (left, right, max_pair)
                });
        let (_, _, max_pair_rtl) =
            s.iter()
                .rev()
                .fold((0, 0, 0), |(mut left, mut right, mut max_pair), p| {
                    match p {
                        b'(' => {
                            left += 1;
                        }
                        b')' => {
                            right += 1;
                        }
                        _ => unreachable!(),
                    }
                    if right == left {
                        max_pair = max_pair.max(right);
                    }
                    if right < left {
                        (left, right) = (0, 0);
                    }
                    (left, right, max_pair)
                });
        max_pair_ltr.max(max_pair_rtl) * 2
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            ("(()", 2),
            (")()())", 4),
            ("", 0),
            (")(((((()())()()))()(()))(", 22),
        ];
        for (input, expected) in tt {
            let output = super::Solution::longest_valid_parentheses(input.into());
            assert_eq!(expected, output);
        }
    }
}

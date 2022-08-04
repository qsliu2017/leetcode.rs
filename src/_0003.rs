struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();
        let s: Vec<char> = s.chars().collect();
        (0..len)
            .fold(
                (std::collections::HashMap::new(), 0, 0),
                |(mut counts, mut from, max), to| {
                    let n = *counts.get(&s[to]).unwrap_or(&0) + 1;
                    counts.insert(s[to], n);
                    if n > 1 {
                        loop {
                            let n = *counts.get(&s[from]).unwrap() - 1;
                            counts.insert(s[from], n);
                            from += 1;
                            if n == 1 {
                                break;
                            }
                        }
                    }
                    (counts, from, max.max(to - from + 1))
                },
            )
            .2 as i32
    }
}
#[cfg(test)]
mod tests {
    use crate::_0003::Solution;

    #[test]
    fn test() {
        let tt = [("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3)];
        for (input, output) in tt {
            assert_eq!(
                Solution::length_of_longest_substring(input.to_string()),
                output
            );
        }
    }
}

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;

        let mut ans = vec![];
        let mut left = 0;
        while left < words.len() {
            let mut right = left;
            let mut chcnt = 0;
            while right < words.len() && chcnt + right - left + words[right].len() <= max_width {
                chcnt += words[right].len();
                right += 1;
            }
            let mut s = String::new();

            // last line
            if right == words.len() {
                s.push_str(&words[left..].join(" "));
            } else {
                for i in 0..right - left {
                    s.push_str(&words[left + i]);
                    if i != right - left - 1 {
                        let space = (max_width - chcnt) / (right - left - 1)
                            + if i < (max_width - chcnt) % (right - left - 1) {
                                1
                            } else {
                                0
                            };
                        s.push_str(&" ".repeat(space));
                    }
                }
            }
            s.push_str(&" ".repeat(max_width - s.len()));
            ans.push(s);
            left = right;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification.",
                ],
                16,
                vec!["This    is    an", "example  of text", "justification.  "],
            ),
            (
                vec!["What", "must", "be", "acknowledgment", "shall", "be"],
                16,
                vec!["What   must   be", "acknowledgment  ", "shall be        "],
            ),
            (
                vec![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do",
                ],
                20,
                vec![
                    "Science  is  what we",
                    "understand      well",
                    "enough to explain to",
                    "a  computer.  Art is",
                    "everything  else  we",
                    "do                  ",
                ],
            ),
        ];
        for (words, max_width, expected) in tt {
            let output = super::Solution::full_justify(
                words.into_iter().map(|s| s.to_string()).collect(),
                max_width,
            );
            assert_eq!(expected, output);
        }
    }
}

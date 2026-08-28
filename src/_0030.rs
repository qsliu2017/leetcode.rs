#[allow(unused)]
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let n_words = words.len();
        let word_len = words[0].len();
        let bytes = s.as_bytes();

        let word_count = words.iter().map(String::as_bytes).fold(
            HashMap::<_, usize>::new(),
            |mut word_count, word| {
                *word_count.entry(word).or_default() += 1;
                word_count
            },
        );

        let n_distinct_words = word_count.len();

        let (word_idx, word_count) = word_count.into_iter().fold(
            (HashMap::new(), vec![0; n_distinct_words]),
            |(mut word_idx, mut word_count), (word, count)| {
                let idx = word_idx.len();
                word_idx.insert(word, idx);
                word_count[idx] = count;
                (word_idx, word_count)
            },
        );

        (0..word_len
            .min(s.len().saturating_sub(n_words * word_len) + 1)
            .max(0))
            .flat_map(|start| {
                let words = bytes[start..].chunks(word_len).collect::<Vec<_>>();
                let mut window_count = words[..n_words - 1].iter().fold(
                    vec![0; word_count.len()],
                    |mut w_word_count, word| {
                        word_idx.get(word).inspect(|&&i| {
                            w_word_count[i] += 1;
                        });
                        w_word_count
                    },
                );
                words
                    .windows(n_words)
                    .enumerate()
                    .filter_map(|(window_idx, words)| {
                        // add last word
                        word_idx.get(words[words.len() - 1]).inspect(|&&i| {
                            window_count[i] += 1;
                        });
                        let match_ = window_count.eq(&word_count);
                        // remove first word
                        word_idx.get(words[0]).inspect(|&&i| {
                            window_count[i] -= 1;
                        });
                        match_.then(|| (start + word_len * window_idx) as i32)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            ("barfoothefoobarman", vec!["foo", "bar"], vec![0, 9]),
            (
                "wordgoodgoodgoodbestword",
                vec!["word", "good", "best", "word"],
                vec![],
            ),
            (
                "barfoofoobarthefoobarman",
                vec!["bar", "foo", "the"],
                vec![6, 9, 12],
            ),
        ];
        for (s, words, expected) in tt {
            let words = words.into_iter().map(|s| s.to_string()).collect();
            let output = super::Solution::find_substring(s.into(), words);
            assert_eq!(expected, output);
        }
    }
}

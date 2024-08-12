use std::collections::{HashMap, HashSet};
struct MagicDictionary {
    n_char_to_words: HashMap<usize, HashSet<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            n_char_to_words: HashMap::new(),
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        dictionary.into_iter().for_each(|word| {
            let n = word.len();
            self.n_char_to_words
                .entry(n)
                .or_insert_with(|| HashSet::new())
                .insert(word);
        });
    }

    fn search(&self, search_word: String) -> bool {
        let n = search_word.len();
        self.n_char_to_words.get(&n).is_some_and(|words| {
            words.into_iter().any(|word| {
                word.as_bytes()
                    .into_iter()
                    .zip(search_word.as_bytes())
                    .map(|(a, b)| if a != b { 1 } else { 0 })
                    .sum::<usize>()
                    == 1
            })
        })
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
#[cfg(test)]
mod tests {
    use super::MagicDictionary;

    #[test]
    fn test() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
        assert_eq!(dict.search("hello".to_string()), false);
        assert_eq!(dict.search("hhllo".to_string()), true);
        assert_eq!(dict.search("hell".to_string()), false);
        assert_eq!(dict.search("leetcoded".to_string()), false);
    }
}

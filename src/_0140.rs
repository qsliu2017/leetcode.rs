#[allow(unused)]
struct Solution;

use std::{
    collections::{HashMap, HashSet},
    iter,
};
#[derive(Debug)]
struct TrieNode {
    is_terminal: bool,
    next: HashMap<u8, TrieNode>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            is_terminal: false,
            next: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &[u8]) {
        let Some((&first, rest)) = word.split_first() else {
            self.is_terminal = true;
            return;
        };
        let next = self.next.entry(first).or_insert_with(|| TrieNode::new());
        next.insert(rest);
    }
}

#[derive(Debug)]
struct Trie {
    root: HashMap<u8, TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: HashMap::new(),
        }
    }
    pub fn insert(&mut self, word: &[u8]) {
        let Some((&first, rest)) = word.split_first() else {
            unreachable!()
        };
        let next = self.root.entry(first).or_insert_with(|| TrieNode::new());
        next.insert(rest);
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let s = s.as_bytes();
        let trie = word_dict.into_iter().fold(Trie::new(), |mut trie, word| {
            trie.insert(word.as_bytes());
            trie
        });
        let mut to_visit = vec![0];
        // for i in back_connections[j], s[i..j] is a word
        let mut back_connections = vec![HashSet::new(); s.len() + 1];
        let visit = |mut i: usize| -> _ {
            let mut current = (i < s.len()).then(|| trie.root.get(&s[i])).flatten();
            iter::from_fn(move || {
                while let Some(node) = current {
                    i += 1;
                    current = (i < s.len()).then(|| node.next.get(&s[i])).flatten();
                    if node.is_terminal {
                        return Some(i);
                    }
                }
                None
            })
        };
        while !to_visit.is_empty() {
            to_visit = to_visit
                .into_iter()
                .flat_map(|i| {
                    visit(i)
                        .inspect(|&j| {
                            back_connections[j].insert(i);
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
        }

        fn dfs(
            j: usize,
            stack: &mut Vec<usize>,
            ans: &mut Vec<String>,
            s: &[u8],
            back_connections: &Vec<HashSet<usize>>,
        ) {
            stack.push(j);
            if j == 0 {
                ans.push(
                    stack
                        .windows(2)
                        .rev()
                        .map(|w| {
                            let &[j, i] = w else { unreachable!() };
                            String::from_utf8_lossy(&s[i..j])
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                );
            } else {
                for &i in &back_connections[j] {
                    dfs(i, stack, ans, s, back_connections);
                }
            }
            stack.pop();
        }

        let mut stack = vec![];
        let mut ans = vec![];
        dfs(s.len(), &mut stack, &mut ans, &s, &back_connections);

        ans
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test() {
        let tt = [
            (
                "catsanddog",
                vec!["cat", "cats", "and", "sand", "dog"],
                vec!["cats and dog", "cat sand dog"],
            ),
            (
                "pineapplepenapple",
                vec!["apple", "pen", "applepen", "pine", "pineapple"],
                vec![
                    "pine apple pen apple",
                    "pineapple pen apple",
                    "pine applepen apple",
                ],
            ),
            (
                "catsandog",
                vec!["cats", "dog", "sand", "and", "cat"],
                vec![],
            ),
        ];
        for (s, word_dict, expected) in tt {
            let output = super::Solution::word_break(
                s.to_string(),
                word_dict.into_iter().map(|s| s.to_string()).collect(),
            );
            assert_eq!(
                expected
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<HashSet<_>>(),
                output.into_iter().collect::<HashSet<_>>()
            );
        }
    }
}

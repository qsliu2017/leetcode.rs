#[allow(unused)]
struct Solution;

use std::collections::{HashMap, HashSet};
struct TrieNode {
    is_end: bool,
    next: HashMap<char, TrieNode>,
}
impl TrieNode {
    pub fn new() -> Self {
        Self {
            is_end: false,
            next: HashMap::new(),
        }
    }
}
struct Trie {
    root: TrieNode,
}
impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }
    pub fn insert(&mut self, word: &String) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.next.entry(ch).or_insert_with(|| TrieNode::new());
        }
        current.is_end = true;
    }
}
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = words.into_iter().fold(Trie::new(), |mut trie, word| {
            trie.insert(&word);
            trie
        });
        let (m, n) = (board.len(), board[0].len());
        let mut stack = vec![];
        let mut ans = HashSet::new();
        let mut visited = vec![vec![false; n]; m];
        fn dfs(
            i: usize,
            j: usize,
            stack: &mut Vec<char>,
            ans: &mut HashSet<String>,
            current: &mut TrieNode,
            visited: &mut Vec<Vec<bool>>,
            board: &Vec<Vec<char>>,
        ) {
            let ch = board[i][j];
            if let Some(current) = current.next.get_mut(&ch) {
                stack.push(ch);
                visited[i][j] = true;

                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if let (Some(x), Some(y)) = (
                        i.checked_add_signed(dx).filter(|&x| x < visited.len()),
                        j.checked_add_signed(dy).filter(|&y| y < visited[0].len()),
                    ) {
                        if !visited[x][y] {
                            dfs(x, y, stack, ans, current, visited, board);
                        }
                    }
                }

                if current.is_end {
                    ans.insert(stack.iter().collect());
                }

                visited[i][j] = false;
                stack.pop();
            }
        }
        for i in 0..m {
            for j in 0..n {
                dfs(
                    i,
                    j,
                    &mut stack,
                    &mut ans,
                    &mut trie.root,
                    &mut visited,
                    &board,
                );
            }
        }
        ans.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v'],
                ],
                vec!["oath", "pea", "eat", "rain"],
                vec!["eat", "oath"],
            ),
            (vec![vec!['a', 'b'], vec!['c', 'd']], vec!["abcb"], vec![]),
        ];
        for (board, words, expected) in tt {
            let output = super::Solution::find_words(
                board,
                words.into_iter().map(|s| s.to_string()).collect(),
            );
            assert_eq!(expected, output);
        }
    }
}

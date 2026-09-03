#[allow(unused)]
struct Solution;
use std::{
    collections::{HashMap, HashSet},
    iter,
};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let dict = word_list
            .iter()
            .enumerate()
            .map(|(i, word)| (word, i))
            .collect::<HashMap<_, _>>();
        let Some(&end_index) = dict.get(&end_word) else {
            return 0;
        };
        let is_connect = |a: &String, b: &String| {
            iter::zip(a.as_bytes(), b.as_bytes())
                .filter(|(a, b)| a != b)
                .count()
                == 1
        };
        let connections = (0..dict.len())
            .map(|i| {
                (0..dict.len())
                    .filter(|&j| i != j && is_connect(&word_list[i], &word_list[j]))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut to_visit = (0..dict.len())
            .filter(|&i| is_connect(&begin_word, &word_list[i]))
            .collect::<Vec<_>>();

        let mut visited = vec![false; dict.len()];

        let mut dist = 1;
        while !to_visit.is_empty() {
            dist += 1;
            for &i in &to_visit {
                visited[i] = true;
            }
            if visited[end_index] {
                return dist;
            }
            to_visit = to_visit
                .into_iter()
                .flat_map(|i| &connections[i])
                .filter(|&&j| !visited[j])
                .collect::<HashSet<_>>()
                .into_iter()
                .copied()
                .collect();
        }
        0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                "hit",
                "cog",
                vec!["hot", "dot", "dog", "lot", "log", "cog"],
                5,
            ),
            ("hit", "cog", vec!["hot", "dot", "dog", "lot", "log"], 0),
        ];
        for (begin_word, end_word, word_list, expected) in tt {
            let output = super::Solution::ladder_length(
                begin_word.to_string(),
                end_word.to_string(),
                word_list.into_iter().map(|s| s.to_string()).collect(),
            );
            assert_eq!(expected, output);
        }
    }
}

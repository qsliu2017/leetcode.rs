#[allow(unused)]
struct Solution;
use std::{
    collections::{HashMap, HashSet},
    iter,
};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let n = word_list.len();
        // dict[word] = index
        let dict = word_list
            .iter()
            .enumerate()
            .map(|(i, word)| (word, i))
            .collect::<HashMap<_, _>>();
        let Some(&end_index) = dict.get(&end_word) else {
            return vec![];
        };

        let is_connected = |a: &String, b: &String| {
            a.as_bytes()
                .iter()
                .zip(b.as_bytes())
                .filter(|(a, b)| a != b)
                .count()
                == 1
        };
        let connected = word_list
            .iter()
            .enumerate()
            .map(|(i, a)| {
                word_list
                    .iter()
                    .enumerate()
                    .map(|(j, b)| i != j && is_connected(a, b))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut previous_indices = vec![vec![]; n];

        let mut to_visit = word_list
            .iter()
            .enumerate()
            .filter_map(|(i, word)| is_connected(&begin_word, word).then_some(i))
            .collect::<HashSet<_>>();

        to_visit.iter().for_each(|&i| {
            previous_indices[i].push(usize::MAX);
        });

        while !to_visit.is_empty() {
            let next_to_visit = to_visit
                .into_iter()
                .map(|i| {
                    (0..n)
                        .filter(|&j| connected[i][j] && previous_indices[j].is_empty())
                        .map(move |j| (i, j))
                        .collect::<Vec<_>>()
                })
                .fold(HashMap::<_, Vec<_>>::new(), |mut acc, connects| {
                    for (i, j) in connects {
                        acc.entry(j).or_default().push(i);
                    }
                    acc
                });

            to_visit = next_to_visit.keys().copied().collect();

            for (i, from_indices) in next_to_visit {
                previous_indices[i] = from_indices;
            }

            if !previous_indices[end_index].is_empty() {
                break;
            }
        }

        fn dfs(
            i: usize,
            stack: &mut Vec<usize>,
            ans: &mut Vec<Vec<String>>,
            word_list: &Vec<String>,
            begin_word: &String,
            previous_indices: &Vec<Vec<usize>>,
        ) {
            if i == usize::MAX {
                ans.push(
                    iter::once_with(|| begin_word.clone())
                        .chain(stack.iter().rev().map(|&j| word_list[j].clone()))
                        .collect(),
                );
                return;
            }
            stack.push(i);
            previous_indices[i]
                .iter()
                .for_each(|&j| dfs(j, stack, ans, word_list, begin_word, previous_indices));
            stack.pop();
        }

        let mut ans = vec![];
        dfs(
            end_index,
            &mut vec![],
            &mut ans,
            &word_list,
            &begin_word,
            &previous_indices,
        );

        ans
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
                vec![
                    vec!["hit", "hot", "dot", "dog", "cog"],
                    vec!["hit", "hot", "lot", "log", "cog"],
                ],
            ),
            (
                "hit",
                "cog",
                vec!["hot", "dot", "dog", "lot", "log"],
                vec![],
            ),
        ];
        for (begin_word, end_word, word_list, expected) in tt {
            let output = super::Solution::find_ladders(
                begin_word.to_string(),
                end_word.to_string(),
                word_list.into_iter().map(|s| s.to_string()).collect(),
            );
            assert_eq!(expected, output);
        }
    }
}

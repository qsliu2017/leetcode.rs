#[allow(unused)]
struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(
                std::collections::HashMap::<String, Vec<String>>::new(),
                |mut dict, s| {
                    let mut bytes = s.clone().into_bytes();
                    bytes.sort();
                    let sorted = String::from_utf8(bytes).unwrap();
                    if let Some(list) = dict.get_mut(&sorted) {
                        list.push(s);
                    } else {
                        dict.insert(sorted, vec![s]);
                    }
                    dict
                },
            )
            .into_iter()
            .map(|(_, list)| list)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec!["eat", "tea", "tan", "ate", "nat", "bat"],
                vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
            ),
            (vec![""], vec![vec![""]]),
            (vec!["a"], vec![vec!["a"]]),
        ];
        for (strs, expected) in tt {
            let output =
                super::Solution::group_anagrams(strs.into_iter().map(|s| s.to_string()).collect());
            assert_eq!(expected, output);
        }
    }
}

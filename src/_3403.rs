#[allow(unused)]
struct Solution;
impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let n = word.len();
        let m = word.len() - num_friends as usize + 1;
        word.bytes()
            .enumerate()
            .map(|(i, _)| &word[i..(i + m).min(n)])
            .max()
            .unwrap()
            .to_string()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("dbca", 2, "dbc"), ("gggg", 4, "g")];
        for (word, num_friends, expected) in tt {
            let output = super::Solution::answer_string(word.to_string(), num_friends);
            assert_eq!(expected, output);
        }
    }
}

struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        (0..n)
            .fold(vec![vec![String::from("")]], |mut groups, _| {
                let g = groups
                    .iter()
                    .skip(1)
                    .zip(groups.iter().rev())
                    .flat_map(|(left, right)| {
                        left.iter().flat_map(move |left| {
                            right.iter().map(move |right| format!("{}{}", left, right))
                        })
                    })
                    .chain(groups.last().unwrap().iter().map(|s| format!("({})", s)))
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                groups.push(g);
                groups
            })
            .last()
            .unwrap()
            .to_vec()
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::_0022::Solution;

    #[test]
    fn test() {
        let tt = [
            (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
            (1, vec!["()"]),
        ];
        for (input, output) in tt {
            assert_eq!(
                Solution::generate_parenthesis(input)
                    .into_iter()
                    .collect::<HashSet<_>>(),
                output
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<HashSet<_>>()
            );
        }
    }
}

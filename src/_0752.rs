struct Solution;
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let rotate = |c: &char| -> Vec<char> {
            let slots = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let i = slots.binary_search(c).unwrap();
            vec![
                slots[if i > 0 { i - 1 } else { slots.len() - 1 }],
                slots[(i + 1) % slots.len()],
            ]
        };
        let rotate_string = |s: &String| -> Vec<String> {
            s.char_indices()
                .flat_map(|(i, c)| {
                    let s = s.clone();
                    rotate(&c).into_iter().map(move |c| {
                        let mut s = s.clone();
                        s.replace_range(i..=i, String::from(c).as_str());
                        s
                    })
                })
                .collect()
        };
        let deadends = deadends
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let mut to_visit = std::collections::HashSet::from([String::from("0000")]);
        let mut visited = std::collections::HashSet::new();
        let mut t = 0;
        while !to_visit.is_empty() {
            to_visit = to_visit
                .into_iter()
                .filter(|s| !deadends.contains(s))
                .filter(|s| !visited.contains(s))
                .collect::<Vec<_>>()
                .into_iter()
                .flat_map(|s| {
                    let fm = rotate_string(&s);
                    visited.insert(s);
                    fm
                })
                .collect();
            if visited.contains(&target) {
                return t;
            } else {
                t += 1;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use crate::_0752::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec!["0201", "0101", "0102", "1212", "2002"], "0202", 6),
            (vec!["8888"], "0009", 1),
            (
                vec![
                    "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
                ],
                "8888",
                -1,
            ),
        ];
        for (deadends, target, output) in tt {
            assert_eq!(
                Solution::open_lock(
                    deadends.into_iter().map(|s| s.to_string()).collect(),
                    target.to_string()
                ),
                output
            );
        }
    }
}

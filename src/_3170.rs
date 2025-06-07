#[allow(unused)]
struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn clear_stars(s: String) -> String {
        let (_, deleted) = s.bytes().into_iter().enumerate().fold(
            (BinaryHeap::with_capacity(s.len()), vec![false; s.len()]),
            |(mut pq, mut deleted), (i, b)| {
                if b == b'*' {
                    pq.pop().inspect(|&(_, i)| deleted[i] = true);
                    deleted[i] = true;
                } else {
                    pq.push((Reverse(b), i));
                }
                (pq, deleted)
            },
        );
        String::from_utf8(
            s.bytes()
                .zip(deleted)
                .filter_map(|(b, deleted)| (!deleted).then_some(b))
                .collect(),
        )
        .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("aaba*", "aab"), ("abc", "abc")];
        for (s, expected) in tt {
            let output = super::Solution::clear_stars(s.to_string());
            assert_eq!(expected, output);
        }
    }
}

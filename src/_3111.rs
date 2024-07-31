#[allow(unused)]
struct Solution;
use std::collections::BTreeSet;
impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        points
            .into_iter()
            .map(|point| point[0])
            .collect::<BTreeSet<_>>()
            .into_iter()
            .fold((0, i32::MIN), |(ans, cover), x| {
                if x > cover {
                    (ans + 1, x + w)
                } else {
                    (ans, cover)
                }
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec![2, 1],
                    vec![1, 0],
                    vec![1, 4],
                    vec![1, 8],
                    vec![3, 5],
                    vec![4, 6],
                ],
                1,
                2,
            ),
            (
                vec![
                    vec![0, 0],
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 3],
                    vec![4, 4],
                    vec![5, 5],
                    vec![6, 6],
                ],
                2,
                3,
            ),
            (vec![vec![2, 3], vec![1, 2]], 0, 2),
        ];
        for (points, w, expected) in tt {
            let output = super::Solution::min_rectangles_to_cover_points(points, w);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
use std::cmp::max;
impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let points = points
            .into_iter()
            .map(|point| (point[0], point[1]))
            .map(|(x, y)| (x + y, x - y))
            .collect::<Vec<_>>();
        let (x_max, x_min, y_max, y_min) = points.iter().fold(
            (
                (&i32::MIN, &i32::MIN),
                (&i32::MAX, &i32::MAX),
                (&i32::MIN, &i32::MIN),
                (&i32::MAX, &i32::MAX),
            ),
            |(x_max, x_min, y_max, y_min), (x, y)| {
                (
                    match (x > x_max.0, x_max.1 > x) {
                        (true, _) => (x, x_max.0),
                        (_, true) => x_max,
                        _ => (x_max.0, x),
                    },
                    match (x < x_min.0, x_min.1 < x) {
                        (true, _) => (x, x_min.0),
                        (_, true) => x_min,
                        _ => (x_min.0, x),
                    },
                    match (y > y_max.0, y_max.1 > y) {
                        (true, _) => (y, y_max.0),
                        (_, true) => y_max,
                        _ => (y_max.0, y),
                    },
                    match (y < y_min.0, y_min.1 < y) {
                        (true, _) => (y, y_min.0),
                        (_, true) => y_min,
                        _ => (y_min.0, y),
                    },
                )
            },
        );
        points
            .iter()
            .map(|(x, y)| {
                max(
                    if x != x_max.0 { x_max.0 } else { x_max.1 }
                        - if x != x_min.0 { x_min.0 } else { x_min.1 },
                    if y != y_max.0 { y_max.0 } else { y_max.1 }
                        - if y != y_min.0 { y_min.0 } else { y_min.1 },
                )
            })
            .min()
            .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]], 12),
            (vec![vec![1, 1], vec![1, 1], vec![1, 1]], 0),
        ];
        for (points, expected) in tt {
            let output = super::Solution::minimum_distance(points);
            assert_eq!(expected, output);
        }
    }
}

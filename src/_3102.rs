#[allow(unused)]
struct Solution;
use std::{
    cmp::{max, Reverse},
    collections::BinaryHeap,
};
impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let points = points
            .into_iter()
            .map(|point| (point[0], point[1]))
            .map(|(x, y)| (x + y, x - y))
            .collect::<Vec<_>>();
        let mut x_max_heap = points.iter().map(|(x, _)| x).collect::<BinaryHeap<_>>();
        let mut x_min_heap = points
            .iter()
            .map(|(x, _)| Reverse(x))
            .collect::<BinaryHeap<_>>();
        let mut y_max_heap = points.iter().map(|(_, y)| y).collect::<BinaryHeap<_>>();
        let mut y_min_heap = points
            .iter()
            .map(|(_, y)| Reverse(y))
            .collect::<BinaryHeap<_>>();
        points
            .iter()
            .map(|(x, y)| {
                let x_max = x_max_heap
                    .peek()
                    .and_then(|&m| (m != x).then_some(m))
                    .unwrap_or_else(|| {
                        x_max_heap.pop();
                        let x_max = *x_max_heap.peek().unwrap();
                        x_max_heap.push(x);
                        x_max
                    });
                let x_min = x_min_heap
                    .peek()
                    .and_then(|&Reverse(m)| (m != x).then_some(m))
                    .unwrap_or_else(|| {
                        x_min_heap.pop();
                        let Reverse(x_min) = *x_min_heap.peek().unwrap();
                        x_min_heap.push(Reverse(x));
                        x_min
                    });
                let y_max = y_max_heap
                    .peek()
                    .and_then(|&m| (m != y).then_some(m))
                    .unwrap_or_else(|| {
                        y_max_heap.pop();
                        let y_max = *y_max_heap.peek().unwrap();
                        y_max_heap.push(y);
                        y_max
                    });
                let y_min = y_min_heap
                    .peek()
                    .and_then(|&Reverse(m)| (m != y).then_some(m))
                    .unwrap_or_else(|| {
                        y_min_heap.pop();
                        let Reverse(y_min) = *y_min_heap.peek().unwrap();
                        y_min_heap.push(Reverse(y));
                        y_min
                    });
                max(x_max - x_min, y_max - y_min)
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

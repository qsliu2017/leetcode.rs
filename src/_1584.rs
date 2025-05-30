#[allow(unused)]
struct Solution;

struct DisjoinUnionSet {
    parent: Vec<usize>,
}

impl DisjoinUnionSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: Vec::from_iter(0..n),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let parent = self.parent[i];
        if parent == i {
            return i;
        }
        let parent = self.find(parent);
        self.parent[i] = parent;
        return parent;
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }
        let (x, y) = (parent_x.min(parent_y), parent_x.max(parent_y));
        self.parent[y] = x;
        true
    }
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .map(|(i, j)| {
                (
                    points[i]
                        .iter()
                        .zip(points[j].iter())
                        .map(|(a, b)| (a - b).abs())
                        .sum::<i32>(),
                    i,
                    j,
                )
            })
            .collect::<Vec<_>>();
        edges.sort_by_key(|&(w, _, _)| w);
        let mut union_set = DisjoinUnionSet::new(n);
        edges
            .into_iter()
            .filter_map(move |(w, i, j)| {
                if union_set.union(i, j) {
                    // connected
                    Some(w)
                } else {
                    None
                }
            })
            .sum::<_>()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]],
                20,
            ),
            (vec![vec![3, 12], vec![-2, 5], vec![-4, 1]], 18),
            (vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]], 4),
            (
                vec![vec![-1000000, -1000000], vec![1000000, 1000000]],
                4000000,
            ),
            (vec![vec![0, 0]], 0),
            (
                vec![
                    vec![-14, -14],
                    vec![-18, 5],
                    vec![18, -10],
                    vec![18, 18],
                    vec![10, -2],
                ],
                102,
            ),
        ];
        for (points, expected) in tt {
            let output = super::Solution::min_cost_connect_points(points);
            assert_eq!(expected, output);
        }
    }
}

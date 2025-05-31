#[allow(unused)]
struct Solution;

use std::collections::HashMap;

struct UnionSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        Self {
            parent: Vec::from_iter(0..n),
            rank: vec![1; n],
        }
    }
    fn find(&mut self, i: usize) -> usize {
        let parent = self.parent[i];
        if parent == i {
            parent
        } else {
            let root = self.find(parent);
            self.parent[i] = root;
            root
        }
    }
    fn union(&mut self, i: usize, j: usize) -> Result<(usize, usize), usize> {
        let x = self.find(i);
        let y = self.find(j);
        if x == y {
            Err(x)
        } else {
            let (x, y) = (x.min(y), x.max(y));
            self.parent[y] = x;
            self.rank[x] += self.rank[y];
            Ok((x, y))
        }
    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut union_set = UnionSet::new(n as usize);
        let (union_set_edge, n_part) = edges.into_iter().fold(
            (HashMap::new(), n),
            |(mut union_set_edge, mut n_part), edge| {
                let [a, b] = edge[..2] else { unreachable!() };
                let (a, b) = (a as usize, b as usize);
                match union_set.union(a, b) {
                    Ok((x, y)) => {
                        *union_set_edge.entry(x).or_insert(0) +=
                            1 + union_set_edge.remove(&y).unwrap_or(0);
                        n_part -= 1;
                    }
                    Err(x) => {
                        union_set_edge.entry(x).and_modify(|cnt| *cnt += 1);
                    }
                }
                (union_set_edge, n_part)
            },
        );
        n_part
            - union_set_edge
                .into_iter()
                .filter(|&(root, n_edge)| {
                    let n_node = union_set.rank[root];
                    n_node * (n_node - 1) / 2 != n_edge as usize
                })
                .count() as i32
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]], 3),
            (
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]],
                1,
            ),
            (
                5,
                vec![
                    vec![1, 2],
                    vec![3, 4],
                    vec![1, 4],
                    vec![2, 3],
                    vec![1, 3],
                    vec![2, 4],
                ],
                2,
            ),
        ];
        for (n, edges, expected) in tt {
            let output = super::Solution::count_complete_components(n, edges);
            assert_eq!(expected, output);
        }
    }
}

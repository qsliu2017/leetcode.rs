#[allow(unused)]
struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut to_visit = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell == 1)
                    .map(move |(j, _)| (i, j))
            })
            .collect::<HashSet<_>>();
        let mut safe = vec![vec![i32::MAX; n]; n];
        let mut t = -1;
        while !to_visit.is_empty() {
            t += 1;
            to_visit.iter().for_each(|&(i, j)| {
                safe[i][j] = t;
            });
            to_visit = to_visit
                .into_iter()
                .flat_map(|(i, j)| {
                    let (i, j) = (i as i32, j as i32);
                    [(1, 0), (-1, 0), (0, 1), (0, -1)]
                        .into_iter()
                        .map(move |(dx, dy)| (i + dx, j + dy))
                        .filter(|&(x, y)| {
                            0 <= x
                                && x < n as i32
                                && 0 <= y
                                && y < n as i32
                                && safe[x as usize][y as usize] == i32::MAX
                        })
                        .map(|(x, y)| (x as usize, y as usize))
                })
                .collect();
        }
        let answer_space = Vec::from_iter(0..=safe[0][0].min(safe[n - 1][n - 1]));
        let pp = answer_space.partition_point(|&ans| {
            let mut union_set = Vec::from_iter(0..n * n);
            fn find(union_set: &mut Vec<usize>, i: usize) -> usize {
                let parent = union_set[i];
                if parent == i {
                    parent
                } else {
                    let root = find(union_set, parent);
                    union_set[i] = root;
                    root
                }
            }
            let mut union = |a: usize, b: usize| {
                let x = find(&mut union_set, a);
                let y = find(&mut union_set, b);
                if x == y {
                    false
                } else {
                    let (x, y) = (x.min(y), x.max(y));
                    union_set[y] = x;
                    true
                }
            };
            safe.iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .filter_map(move |(j, &cell)| (cell >= ans).then_some((i, j)))
                })
                .for_each(|(i, j)| {
                    if i > 0 && safe[i - 1][j] >= ans {
                        union(i * n + j, (i - 1) * n + j);
                    }
                    if j > 0 && safe[i][j - 1] >= ans {
                        union(i * n + j, i * n + j - 1);
                    }
                });
            find(&mut union_set, 0) == find(&mut union_set, n * n - 1)
        });
        if pp > 0 {
            answer_space[pp - 1]
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]], 0),
            (vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]], 2),
            (
                vec![
                    vec![0, 0, 0, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![1, 0, 0, 0],
                ],
                2,
            ),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::maximum_safeness_factor(grid);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![-1; n]; m];
        let mut row_pqs = vec![BinaryHeap::<Reverse<(i32, usize)>>::new(); m];
        let mut col_pqs = vec![BinaryHeap::<Reverse<(i32, usize)>>::new(); n];
        dp[0][0] = 1;
        row_pqs[0].push(Reverse((1, 0)));
        col_pqs[0].push(Reverse((1, 0)));
        (0..m)
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .skip(1)
            .for_each(|(i, j)| {
                while let Some(&Reverse((_, j_))) = row_pqs[i].peek() {
                    if grid[i][j_] as usize + j_ < j {
                        row_pqs[i].pop();
                    } else {
                        break;
                    }
                }
                while let Some(&Reverse((_, i_))) = col_pqs[j].peek() {
                    if grid[i_][j] as usize + i_ < i {
                        col_pqs[j].pop();
                    } else {
                        break;
                    }
                }
                dp[i][j] = match (row_pqs[i].peek(), col_pqs[j].peek()) {
                    (Some(&Reverse((t1, _))), Some(&Reverse((t2, _)))) => t1.min(t2) + 1,
                    (None, Some(&Reverse((t, _)))) | (Some(&Reverse((t, _))), None) => t + 1,
                    (None, None) => {
                        return;
                    }
                };
                row_pqs[i].push(Reverse((dp[i][j], j)));
                col_pqs[j].push(Reverse((dp[i][j], i)));
            });
        dp[m - 1][n - 1]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec![3, 4, 2, 1],
                    vec![4, 2, 3, 1],
                    vec![2, 1, 0, 0],
                    vec![2, 4, 0, 0],
                ],
                4,
            ),
            (
                vec![
                    vec![3, 4, 2, 1],
                    vec![4, 2, 1, 1],
                    vec![2, 1, 1, 0],
                    vec![3, 4, 1, 0],
                ],
                3,
            ),
            (vec![vec![2, 1, 0], vec![1, 0, 0]], -1),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::minimum_visited_cells(grid);
            assert_eq!(expected, output);
        }
    }
}

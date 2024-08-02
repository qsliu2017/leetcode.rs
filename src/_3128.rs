#[allow(unused)]
struct Solution;
impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let m = grid[0].len();
        let (row_cnt, col_cnt) = grid.iter().enumerate().fold(
            (vec![0; n], vec![0; m]),
            |(row_cnt, col_cnt), (i, row)| {
                row.iter().enumerate().filter(|&(_, &e)| e == 1).fold(
                    (row_cnt, col_cnt),
                    |(mut row_cnt, mut col_cnt), (j, _)| {
                        row_cnt[i] += 1;
                        col_cnt[j] += 1;
                        (row_cnt, col_cnt)
                    },
                )
            },
        );
        grid.into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter(|&(_, e)| e == 1)
                    .map(|(j, _)| (row_cnt[i] - 1) * (col_cnt[j] - 1))
                    .sum::<i64>()
            })
            .sum::<_>()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]], 2),
            (
                vec![vec![1, 0, 0, 0], vec![0, 1, 0, 1], vec![1, 0, 0, 0]],
                0,
            ),
            (vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]], 2),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::number_of_right_triangles(grid);
            assert_eq!(expected, output);
        }
    }
}

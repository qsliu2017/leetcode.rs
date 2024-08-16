#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        grid.into_iter()
            .fold((i32::MIN, vec![i32::MAX; n]), |(ans, mut col_min), row| {
                (
                    col_min
                        .iter_mut()
                        .zip(row.iter())
                        .fold((i32::MAX, ans), |(min, ans), (col_min, &v)| {
                            let ans = ans.max(v - min).max(v - *col_min);
                            *col_min = v.min(min).min(*col_min);
                            (*col_min, ans)
                        })
                        .1,
                    col_min,
                )
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
                    vec![9, 5, 7, 3],
                    vec![8, 9, 6, 1],
                    vec![6, 7, 14, 3],
                    vec![2, 5, 3, 1],
                ],
                9,
            ),
            (vec![vec![4, 3, 2], vec![3, 2, 1]], -1),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::max_score(grid);
            assert_eq!(expected, output);
        }
    }
}

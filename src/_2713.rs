#[allow(unused)]
struct Solution;
use std::collections::BTreeMap;
impl Solution {
    pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        mat.into_iter()
            .enumerate()
            .flat_map(|(i, row)| row.into_iter().enumerate().map(move |(j, v)| (v, i, j)))
            .fold(
                BTreeMap::<_, Vec<_>>::new(),
                |mut v_to_points, (v, i, j)| {
                    v_to_points.entry(v).or_default().push((i, j));
                    v_to_points
                },
            )
            .into_iter()
            .rev()
            .fold(
                (0, vec![0; m], vec![0; n]),
                |(ans, row_max, col_max), (_, points)| {
                    points
                        .into_iter()
                        .map(|(i, j)| (row_max[i].max(col_max[j]) + 1, i, j))
                        .collect::<Vec<_>>()
                        .into_iter()
                        .fold(
                            (ans, row_max, col_max),
                            |(ans, mut row_max, mut col_max), (max, i, j)| {
                                row_max[i] = row_max[i].max(max);
                                col_max[j] = col_max[j].max(max);
                                (ans.max(max), row_max, col_max)
                            },
                        )
                },
            )
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![3, 1], vec![3, 4]], 2),
            (vec![vec![1, 1], vec![1, 1]], 1),
            (vec![vec![3, 1, 6], vec![-9, 5, 7]], 4),
        ];
        for (input, expected) in tt {
            let output = super::Solution::max_increasing_cells(input);
            assert_eq!(expected, output);
        }
    }
}

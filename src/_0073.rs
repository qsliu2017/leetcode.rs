struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (rows, cols) = matrix.iter().enumerate().fold(
            (vec![false; matrix.len()], vec![false; matrix[0].len()]),
            |(rows, cols), (i, row)| {
                row.iter().enumerate().filter(|(_, &v)| v == 0).fold(
                    (rows, cols),
                    |(mut rows, mut cols), (j, _)| {
                        rows[i] = true;
                        cols[j] = true;
                        (rows, cols)
                    },
                )
            },
        );
        matrix.iter_mut().zip(rows.iter()).for_each(|(row, &r)| {
            row.iter_mut()
                .zip(cols.iter())
                .filter(|(_, &c)| c || r)
                .for_each(|(v, _)| {
                    *v = 0;
                });
        });
    }
}
#[cfg(test)]
mod tests {
    use crate::_0073::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
                vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]],
            ),
            (
                vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
                vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
            ),
        ];
        for (mut input, output) in tt {
            Solution::set_zeroes(&mut input);
            assert_eq!(input, output);
        }
    }
}

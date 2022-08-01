struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix
            .iter()
            .flatten()
            .collect::<Vec<_>>()
            .binary_search(&&target)
            .is_ok()
    }

    fn search_matrix_inplace(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        (0..m * n)
            .collect::<Vec<_>>()
            .binary_search_by(|&idx| {
                let (i, j) = (idx / n, idx % n);
                matrix[i][j].cmp(&target)
            })
            .is_ok()
    }
}
#[cfg(test)]
mod tests {
    use crate::_0074::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3,
                true,
            ),
            (
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13,
                false,
            ),
            (vec![vec![1, 1]], 0, false),
        ];
        for (matrix, target, output) in tt {
            assert_eq!(Solution::search_matrix(matrix, target), output);
        }
    }
}

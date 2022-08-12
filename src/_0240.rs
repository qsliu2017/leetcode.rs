struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut i, mut j) = (0, n - 1);
        loop {
            if matrix[i][j] == target {
                return true;
            } else if matrix[i][j] > target && j > 0 {
                j -= 1;
            } else if matrix[i][j] < target && i + 1 < m {
                i += 1;
            } else {
                return false;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::_0240::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30],
                ],
                5,
                true,
            ),
            (
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30],
                ],
                20,
                false,
            ),
        ];
        for (matrix, target, output) in tt {
            assert_eq!(Solution::search_matrix(matrix, target), output);
        }
    }
}

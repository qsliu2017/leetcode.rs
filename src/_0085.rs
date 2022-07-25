struct Solution;
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut above_ones = vec![vec![0; n]; m];
        for j in 0..n {
            above_ones[0][j] = if matrix[0][j] == '1' { 1 } else { 0 };
        }
        for i in 1..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    above_ones[i][j] = 1 + above_ones[i - 1][j];
                }
            }
        }

        (0..m)
            .map(|i| {
                let mut stack = vec![];
                let mut left = vec![0; n];
                for j in 0..n {
                    while let Some(top) = stack.last() {
                        if above_ones[i][*top] >= above_ones[i][j] {
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                    left[j] = stack.last().and_then(|&v| Some(v + 1)).unwrap_or(0);
                    stack.push(j);
                }
                stack.clear();
                let mut right = vec![0; n];
                for j in (0..n).rev() {
                    while let Some(top) = stack.last() {
                        if above_ones[i][*top] >= above_ones[i][j] {
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                    right[j] = stack.last().and_then(|&v| Some(v)).unwrap_or(n);
                    stack.push(j);
                }

                (0..n)
                    .map(|j| (right[j] as i32 - left[j] as i32) * above_ones[i][j])
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use crate::_0085::Solution;

    #[test]
    fn test() {
        let tt = [
            // (vec![vec!['0']], 0),
            // (vec![vec!['1']], 1),
            (
                vec![
                    vec!['1', '0', '1', '0', '0'],
                    vec!['1', '0', '1', '1', '1'],
                    vec!['1', '1', '1', '1', '1'],
                    vec!['1', '0', '0', '1', '0'],
                ],
                6,
            ),
            (vec![vec!['0', '1']], 1),
        ];
        for (input, output) in tt {
            assert_eq!(Solution::maximal_rectangle(input), output);
        }
    }
}

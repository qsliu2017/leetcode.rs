use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());

        let mut to_visit = (0..m)
            .into_iter()
            .flat_map(|i| vec![(i, 0), (i, n - 1)])
            .chain((0..n).into_iter().flat_map(|j| vec![(0, j), (m - 1, j)]))
            .filter(|&(i, j)| board[i][j] == 'O')
            .collect::<HashSet<_>>();
        let mut not_captured = to_visit.clone();

        while !to_visit.is_empty() {
            to_visit = to_visit
                .into_iter()
                .flat_map(|(i, j)| {
                    let mut v = vec![];
                    if i > 0 {
                        v.push((i - 1, j));
                    }
                    if i + 1 < m {
                        v.push((i + 1, j));
                    }
                    if j > 0 {
                        v.push((i, j - 1));
                    }
                    if j + 1 < n {
                        v.push((i, j + 1));
                    }
                    v
                })
                .filter(|t| !not_captured.contains(t))
                .filter(|&(i, j)| board[i][j] == 'O')
                .collect();
            not_captured = not_captured.union(&to_visit).map(|&t| t).collect();
        }

        board
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|r| *r = 'X'));
        not_captured
            .into_iter()
            .for_each(|(i, j)| board[i][j] = 'O');
    }
}
#[cfg(test)]
mod tests {
    use crate::_0130::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec!['X', 'X', 'X', 'X'],
                    vec!['X', 'O', 'O', 'X'],
                    vec!['X', 'X', 'O', 'X'],
                    vec!['X', 'O', 'X', 'X'],
                ],
                vec![
                    vec!['X', 'X', 'X', 'X'],
                    vec!['X', 'X', 'X', 'X'],
                    vec!['X', 'X', 'X', 'X'],
                    vec!['X', 'O', 'X', 'X'],
                ],
            ),
            (vec![vec!['X']], vec![vec!['X']]),
        ];
        for (mut input, output) in tt {
            Solution::solve(&mut input);
            assert_eq!(input, output);
        }
    }
}

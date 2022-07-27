struct Solution;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board.clone();
        let word = word.chars().collect();
        (0..board.len()).any(|i| {
            (0..board[0].len()).any(|j| Self::solve(&mut board, i as i32, j as i32, &word, 0))
        })
    }
    fn solve(board: &mut Vec<Vec<char>>, i: i32, j: i32, word: &Vec<char>, idx: usize) -> bool {
        if idx == word.len() {
            return true;
        }
        if i < 0 || j < 0 {
            return false;
        }
        let (r, c) = (i as usize, j as usize);
        if r >= board.len() || c >= board[0].len() {
            return false;
        }
        let ch = board[r][c];
        if ch == word[idx] {
            board[r][c] = 0 as char;
            for (r, c) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if Self::solve(board, r, c, word, idx + 1) {
                    return true;
                }
            }
            board[r][c] = ch;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::_0079::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED",
                true,
            ),
            (
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "SEE",
                true,
            ),
            (
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCB",
                false,
            ),
        ];
        for (board, word, output) in tt {
            assert_eq!(Solution::exist(board, word.to_string()), output);
        }
    }
}

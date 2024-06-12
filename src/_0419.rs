#[allow(unused)]
struct Solution;
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        board
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &slot)| slot == 'X')
                    .map(|(j, _)| {
                        if (i > 0 && board[i - 1][j] == 'X') || (j > 0 && board[i][j - 1] == 'X') {
                            0
                        } else {
                            1
                        }
                    })
                    .sum::<i32>()
            })
            .sum()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec!['X', '.', '.', 'X'],
                    vec!['.', '.', '.', 'X'],
                    vec!['.', '.', '.', 'X'],
                ],
                2,
            ),
            (vec![vec!['.']], 0),
        ];
        for (input, expected) in tt {
            let output = super::Solution::count_battleships(input);
            assert_eq!(expected, output);
        }
    }
}

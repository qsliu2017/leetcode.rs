struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut to_visit = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == '1')
                    .map(|(j, _)| (i, j))
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();

        let mut cnt = 0;
        while let Some(origin) = to_visit.iter().next().cloned() {
            let mut next_to_visit = vec![origin];
            while let Some((i, j)) = next_to_visit.pop() {
                to_visit.take(&(i, j));
                if i > 0 && to_visit.contains(&(i - 1, j)) {
                    next_to_visit.push((i - 1, j));
                }
                if j > 0 && to_visit.contains(&(i, j - 1)) {
                    next_to_visit.push((i, j - 1));
                }
                if i + 1 < m && to_visit.contains(&(i + 1, j)) {
                    next_to_visit.push((i + 1, j));
                }
                if j + 1 < n && to_visit.contains(&(i, j + 1)) {
                    next_to_visit.push((i, j + 1));
                }
            }
            cnt += 1;
        }

        cnt
    }
}
#[cfg(test)]
mod tests {
    use crate::_0200::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec!['1', '1', '1', '1', '0'],
                    vec!['1', '1', '0', '1', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '0', '0', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['0', '0', '0', '1', '1'],
                ],
                3,
            ),
            (
                vec![
                    vec!['1', '1', '1'],
                    vec!['0', '1', '0'],
                    vec!['1', '1', '1'],
                ],
                1,
            ),
        ];
        for (input, output) in tt {
            assert_eq!(Solution::num_islands(input), output);
        }
    }
}

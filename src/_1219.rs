#[allow(unused)]
struct Solution;
impl Solution {
    fn dfs(
        grid: &Vec<Vec<i32>>,
        row_idx: usize,
        col_idx: usize,
        visited: &mut Vec<Vec<bool>>,
        acc_gold: i32,
        maximum_gold: &mut i32,
    ) {
        visited[row_idx][col_idx] = true;
        let acc_gold = acc_gold + grid[row_idx][col_idx];
        *maximum_gold = acc_gold.max(*maximum_gold);
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x = row_idx as i32 + dx;
            let y = col_idx as i32 + dy;
            if x < 0
                || x >= grid.len() as i32
                || y < 0
                || y >= grid[0].len() as i32
                || visited[x as usize][y as usize]
                || grid[x as usize][y as usize] == 0
            {
                continue;
            }
            Self::dfs(
                grid,
                x as usize,
                y as usize,
                visited,
                acc_gold,
                maximum_gold,
            );
        }
        visited[row_idx][col_idx] = false;
    }
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut maximum_gold = 0;
        grid.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &e)| e != 0)
                .for_each(|(col_idx, _)| {
                    Self::dfs(&grid, row_idx, col_idx, &mut visited, 0, &mut maximum_gold)
                });
        });
        maximum_gold
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]], 24),
            (
                vec![
                    vec![1, 0, 7],
                    vec![2, 0, 6],
                    vec![3, 4, 5],
                    vec![0, 3, 0],
                    vec![9, 0, 20],
                ],
                28,
            ),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::get_maximum_gold(grid);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;

impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize) -> i32 {
        let v = grid[i][j];
        if v == 0 {
            return 0;
        }
        grid[i][j] = 0;

        v + [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(dx, dy)| (i as i32 + dx, j as i32 + dy))
            .filter(|&(x, y)| 0 <= x && x < m as i32 && 0 <= y && y < n as i32)
            .map(|(x, y)| Self::dfs(grid, m, n, x as usize, y as usize))
            .sum::<i32>()
    }
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut grid = grid;
        (0..m)
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .map(|(i, j)| Self::dfs(&mut grid, m, n, i, j))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec![0, 2, 1, 0],
                    vec![4, 0, 0, 3],
                    vec![1, 0, 0, 4],
                    vec![0, 3, 2, 0],
                ],
                7,
            ),
            (
                vec![
                    vec![1, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 1],
                ],
                1,
            ),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::find_max_fish(grid);
            assert_eq!(expected, output);
        }
    }
}

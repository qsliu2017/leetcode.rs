#[allow(unused)]
struct Solution;
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        (0..n)
            .into_iter()
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .map(|(i, j)| {
                (if i == 0 {
                    grid[i][j]
                } else {
                    (grid[i - 1][j] - grid[i][j]).abs()
                }) + (if j == 0 {
                    grid[i][j]
                } else {
                    (grid[i][j - 1] - grid[i][j]).abs()
                }) + (if grid[i][j] == 0 { 0 } else { 2 })
                    + (if i == n - 1 { grid[i][j] } else { 0 })
                    + (if j == n - 1 { grid[i][j] } else { 0 })
            })
            .sum()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![1, 2], vec![3, 4]], 34),
            (vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]], 32),
            (vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]], 46),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::surface_area(grid);
            assert_eq!(expected, output);
        }
    }
}

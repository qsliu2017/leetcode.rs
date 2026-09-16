#[allow(unused)]
struct Solution;
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut to_visit = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(j, &cell)| (cell == 2).then_some((i, j)))
            })
            .collect::<Vec<_>>();
        let mut time = 0;
        while !to_visit.is_empty() {
            to_visit = to_visit
                .into_iter()
                .flat_map(|(i, j)| {
                    [(0, 1), (1, 0), (-1, 0), (0, -1)].into_iter().filter_map(
                        move |(dx, dy)| match (
                            i.checked_add_signed(dx).filter(|&x| x < m),
                            j.checked_add_signed(dy).filter(|&y| y < n),
                        ) {
                            (Some(x), Some(y)) => Some((x, y)),
                            _ => None,
                        },
                    )
                })
                .filter(|&(x, y)| {
                    if grid[x][y] == 1 {
                        grid[x][y] = 2;
                        true
                    } else {
                        false
                    }
                })
                .collect();
            if !to_visit.is_empty() {
                time += 1;
            }
        }
        if grid
            .into_iter()
            .flat_map(|row| row.into_iter())
            .any(|cell| cell == 1)
        {
            -1
        } else {
            time
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]], 4),
            (vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]], -1),
            (vec![vec![0, 2]], 0),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::oranges_rotting(grid);
            assert_eq!(expected, output);
        }
    }
}

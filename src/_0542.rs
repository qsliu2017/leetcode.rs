#[allow(unused)]
struct Solution;
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut mat = mat;
        let mut out = vec![vec![0; n]; m];
        let mut to_visit = mat
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(j, &cell)| (cell == 0).then_some((i, j)))
            })
            .collect::<Vec<_>>();
        let mut d = 1;
        while !to_visit.is_empty() {
            to_visit = to_visit
                .into_iter()
                .flat_map(|(i, j)| {
                    [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().filter_map(
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
                    if mat[x][y] == 1 {
                        mat[x][y] = 0;
                        out[x][y] = d;
                        true
                    } else {
                        false
                    }
                })
                .collect();
            d += 1;
        }
        out
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
            ),
            (
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]],
            ),
        ];
        for (mat, expected) in tt {
            let output = super::Solution::update_matrix(mat);
            assert_eq!(expected, output);
        }
    }
}

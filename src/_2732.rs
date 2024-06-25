#[allow(unused)]
struct Solution;
impl Solution {
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mem = [-1; 1 << 5];
        for (i, v) in grid
            .into_iter()
            .map(|row| row.into_iter().reduce(|acc, b| (acc << 1) + b).unwrap())
            .enumerate()
        {
            if v == 0 {
                return vec![i as _];
            }
            if let Some(r) = (0..1 << 5)
                .filter(|&r| r & v == 0 && mem[r as usize] > -1)
                .next()
            {
                return vec![mem[r as usize] as _, i as _];
            }
            mem[v as usize] = i as _;
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 1, 1, 1]],
                vec![0, 1],
            ),
            (vec![vec![0]], vec![0]),
            (vec![vec![1, 1, 1], vec![1, 1, 1]], vec![]),
        ];
        for (grid, expected) in tt {
            let output = super::Solution::good_subsetof_binary_matrix(grid);
            assert_eq!(expected, output);
        }
    }
}

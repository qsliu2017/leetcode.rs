struct Solution;
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) {
            if !visited[i] {
                visited[i] = true;
                (0..is_connected.len())
                    .into_iter()
                    .filter(|&j| is_connected[i][j] == 1)
                    .for_each(|j| dfs(is_connected, visited, j));
            }
        }

        let mut cnt = 0;
        while let Some(next) = visited
            .iter()
            .enumerate()
            .filter(|&(_, v)| !v)
            .map(|(i, _)| i)
            .next()
        {
            dfs(&is_connected, &mut visited, next);
            cnt += 1;
        }

        cnt
    }
}
#[cfg(test)]
mod tests {
    use crate::_0547::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]], 2),
            (vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]], 3),
        ];
        for (input, output) in tt {
            assert_eq!(Solution::find_circle_num(input), output);
        }
    }
}

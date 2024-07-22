#[allow(unused)]
struct Solution;
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let bombs = bombs
            .into_iter()
            .map(|bomb| (bomb[0] as i128, bomb[1] as i128, (bomb[2] as i128).pow(2)))
            .collect::<Vec<_>>();
        let graph = bombs.iter().enumerate().fold(
            vec![vec![false; n]; n],
            |graph, (i, &(xi, yi, risqaure))| {
                bombs.iter().enumerate().skip(i + 1).fold(
                    graph,
                    |mut graph, (j, &(xj, yj, rjsquare))| {
                        let dsqaure = (xi - xj).pow(2) + (yi - yj).pow(2);
                        if dsqaure <= risqaure {
                            graph[i][j] = true;
                        }
                        if dsqaure <= rjsquare {
                            graph[j][i] = true;
                        }
                        graph
                    },
                )
            },
        );
        graph
            .iter()
            .enumerate()
            .map(|(i, connections)| {
                let mut visited = vec![false; n];
                visited[i] = true;
                let mut to_visit = connections
                    .iter()
                    .enumerate()
                    .filter(|(_, &connected)| connected)
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();
                while !to_visit.is_empty() {
                    to_visit = to_visit
                        .iter()
                        .fold(vec![false; n], |next, &i| {
                            visited[i] = true;
                            graph[i]
                                .iter()
                                .enumerate()
                                .filter(|&(j, &connected)| connected && !visited[j])
                                .fold(next, |mut next, (j, _)| {
                                    next[j] = true;
                                    next
                                })
                        })
                        .iter()
                        .enumerate()
                        .filter(|(_, &connected)| connected)
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();
                }
                visited.into_iter().filter(|&v| v).count() as i32
            })
            .max()
            .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![2, 1, 3], vec![6, 1, 4]], 2),
            (vec![vec![1, 1, 5], vec![10, 10, 5]], 1),
            (
                vec![
                    vec![1, 2, 3],
                    vec![2, 3, 1],
                    vec![3, 4, 2],
                    vec![4, 5, 3],
                    vec![5, 6, 4],
                ],
                5,
            ),
        ];
        for (bombs, expected) in tt {
            let output = super::Solution::maximum_detonation(bombs);
            assert_eq!(expected, output);
        }
    }
}

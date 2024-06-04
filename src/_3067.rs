#[allow(unused)]
struct Solution;
use std::{collections::HashMap, iter::once};
#[allow(dead_code)]
impl Solution {
    fn dfs(
        connections: &Vec<HashMap<usize, i32>>,
        i: usize,
        parent: usize,
        depth: i32,
        signal_speed: i32,
    ) -> HashMap<i32, i32> /* distance -> count */ {
        connections[i]
            .iter()
            .filter(|(&j, _)| j != parent)
            .map(|(&j, &distance)| Self::dfs(connections, j, i, depth + distance, signal_speed))
            .fold(
                HashMap::from_iter(once((depth % signal_speed, 1)).into_iter()),
                |all, branch| {
                    branch.into_iter().fold(all, |mut all, (distance, count)| {
                        all.entry(distance % signal_speed)
                            .and_modify(|c| *c += count)
                            .or_insert(count);
                        all
                    })
                },
            )
    }
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let n = edges.len() + 1;

        // connections[a] = {b: weight}
        let connections = edges.into_iter().fold(
            vec![HashMap::<usize, i32>::new(); n],
            |mut connections, edge| {
                let (a, b, distance) = (edge[0] as usize, edge[1] as usize, edge[2]);
                connections[a].insert(b, distance);
                connections[b].insert(a, distance);
                connections
            },
        );

        connections
            .iter()
            .enumerate()
            .map(|(c, branches)| {
                branches
                    .iter()
                    .map(|(&a, &distance)| Self::dfs(&connections, a, c, distance, signal_speed))
                    .fold((0, 0), |(ans, count), branch| {
                        let c = branch.get(&0).copied().unwrap_or(0);
                        (ans + c * count, count + c)
                    })
                    .0
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 5],
                    vec![2, 3, 13],
                    vec![3, 4, 9],
                    vec![4, 5, 2],
                ],
                1,
                vec![0, 4, 6, 6, 4, 0],
            ),
            (
                vec![
                    vec![0, 6, 3],
                    vec![6, 5, 3],
                    vec![0, 3, 1],
                    vec![3, 2, 7],
                    vec![3, 1, 6],
                    vec![3, 4, 2],
                ],
                3,
                vec![2, 0, 0, 0, 0, 0, 2],
            ),
        ];
        for (edges, signal_speed, expected) in tt {
            let output =
                super::Solution::count_pairs_of_connectable_servers(dbg!(edges), signal_speed);
            assert_eq!(output, expected);
        }
    }
}

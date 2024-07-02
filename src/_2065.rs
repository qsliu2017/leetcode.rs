#[allow(unused)]
struct Solution;
use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};
impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let n = values.len();
        let connection =
            edges
                .into_iter()
                .fold(vec![vec![i32::MAX; n]; n], |mut connection, edge| {
                    let (u, v, time) = (edge[0] as usize, edge[1] as usize, edge[2]);
                    connection[u][v] = time;
                    connection[v][u] = time;
                    connection
                });
        let mut initial = [false; 1000];
        initial[0] = true;
        let mut dp = HashMap::new();
        dp.insert((initial, 0), 0);
        let mut to_visit = vec![(initial, 0)];
        let mut ans = values[0];
        while !to_visit.is_empty() {
            to_visit = to_visit
                .into_iter()
                .fold(HashSet::new(), |next, (visited, i)| {
                    let time = dp.get(&(visited, i)).copied().unwrap();
                    connection[i]
                        .iter()
                        .enumerate()
                        .filter(|(_, &t)| t < i32::MAX && time + t <= max_time)
                        .fold(next, |mut next, (j, &t)| {
                            let mut visited = visited;
                            visited[j] = true;
                            if j == 0 {
                                ans = ans.max(
                                    zip(values.iter(), visited)
                                        .filter(|&(_, v)| v)
                                        .map(|(v, _)| v)
                                        .sum(),
                                );
                            }
                            let acc = dp.entry((visited, j)).or_insert(i32::MAX);
                            if *acc > time + t {
                                *acc = time + t;
                                next.insert((visited, j));
                            }
                            next
                        })
                })
                .into_iter()
                .collect();
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![0, 32, 10, 43],
                vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]],
                49,
                75,
            ),
            (
                vec![5, 10, 15, 20],
                vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 3, 10]],
                30,
                25,
            ),
            (
                vec![1, 2, 3, 4],
                vec![
                    vec![0, 1, 10],
                    vec![1, 2, 11],
                    vec![2, 3, 12],
                    vec![1, 3, 13],
                ],
                50,
                7,
            ),
            (vec![0, 1, 2], vec![vec![1, 2, 10]], 10, 0),
        ];
        for (values, edges, max_time, expected) in tt {
            let output = super::Solution::maximal_path_quality(values, edges, max_time);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;

fn build_nearby(edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let n = edges.len() + 1;
    let mut nearby = vec![Vec::with_capacity(n - 1); n];
    for edge in edges {
        let [u, v] = edge[..2] else { unreachable!() };
        let (u, v) = (u as usize, v as usize);
        nearby[u].push(v);
        nearby[v].push(u);
    }
    nearby
}

fn count_k_nearby(nearby: &Vec<Vec<usize>>, i: usize, k: i32, parent: usize) -> i32 {
    if k < 0 {
        0
    } else if k == 0 {
        1
    } else {
        let mut acc = 1;
        for &j in &nearby[i] {
            if j != parent {
                acc += count_k_nearby(nearby, j, k - 1, i);
            }
        }
        acc
    }
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let nearby1 = build_nearby(&edges1);
        let nearby2 = build_nearby(&edges2);
        let m = edges2.len() + 1;
        let addition = (0..edges2.len() + 1)
            .map(|i| count_k_nearby(&nearby2, i, k - 1, m))
            .max()
            .unwrap();
        let n = edges1.len() + 1;
        (0..n)
            .map(|i| addition + count_k_nearby(&nearby1, i, k, n))
            .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 7],
                    vec![1, 4],
                    vec![4, 5],
                    vec![4, 6],
                ],
                2,
                vec![9, 7, 9, 8, 8],
            ),
            (
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                1,
                vec![6, 3, 3, 3, 3],
            ),
        ];
        for (edges1, edges2, k, expected) in tt {
            let output = super::Solution::max_target_nodes(edges1, edges2, k);
            assert_eq!(expected, output);
        }
    }
}

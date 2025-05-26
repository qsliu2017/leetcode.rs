use std::collections::HashMap;

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        // nodes[node_idx] = color_idx
        let (color_idx_map, nodes) = colors.bytes().fold(
            (HashMap::new(), Vec::with_capacity(colors.len())),
            |(mut color_idx_map, mut node_color), b| {
                let b = b as i32;
                node_color.push(if let Some(&idx) = color_idx_map.get(&b) {
                    idx
                } else {
                    let idx = color_idx_map.len();
                    color_idx_map.insert(b, idx);
                    idx
                });
                (color_idx_map, node_color)
            },
        );
        let n_color = color_idx_map.len();

        // node_inedge[node_idx] = # of in-edge
        let (mut n_in_edges, to_nodes) = edges.into_iter().fold(
            (vec![0; nodes.len()], vec![vec![]; nodes.len()]),
            |(mut n_in_edges, mut to_nodes), edge| {
                let (a, b) = (edge[0] as usize, edge[1] as usize);
                to_nodes[a].push(b);
                n_in_edges[b] += 1;
                (n_in_edges, to_nodes)
            },
        );

        let mut to_visit = n_in_edges
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| (n == 0).then_some(i))
            .collect::<Vec<_>>();
        let mut dp = vec![vec![0; n_color]; nodes.len()]; // dp[node_idx][color_idx] = max count of color from this node
        let mut n_visited = 0;
        let mut ans = 0;
        while to_visit.len() > 0 {
            let mut next_to_visit = Vec::new();
            to_visit.into_iter().for_each(|node_idx| {
                n_visited += 1;
                dp[node_idx][nodes[node_idx]] += 1;
                ans = ans.max(*dp[node_idx].iter().max().unwrap());

                for &out_node_idx in &to_nodes[node_idx] {
                    for color_idx in 0..n_color {
                        dp[out_node_idx][color_idx] =
                            dp[out_node_idx][color_idx].max(dp[node_idx][color_idx]);
                    }
                    n_in_edges[out_node_idx] -= 1;
                    if n_in_edges[out_node_idx] == 0 {
                        next_to_visit.push(out_node_idx);
                    }
                }
            });
            to_visit = next_to_visit;
        }
        if n_visited < nodes.len() {
            -1
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                "abaca",
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]],
                3,
            ),
            ("a", vec![vec![0, 0]], -1),
        ];
        for (colors, edges, expected) in tt {
            let output = super::Solution::largest_path_value(colors.to_string(), edges);
            assert_eq!(expected, output);
        }
    }
}

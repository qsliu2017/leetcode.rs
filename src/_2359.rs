#[allow(unused)]
struct Solution;
impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let mut dist1 = vec![-1; edges.len()];
        let mut dist2 = vec![-1; edges.len()];
        let mut dist = 0;
        let (mut node1, mut node2) = (node1, node2);
        let mut ans = (None, None);
        loop {
            if node1 == -1 && node2 == -1 {
                return -1;
            }
            if node1 != -1 {
                if dist1[node1 as usize] != -1 {
                    // cycle
                    node1 = -1;
                } else if dist2[node1 as usize] != -1 {
                    // common node
                    ans.0 = Some(node1);
                } else {
                    dist1[node1 as usize] = dist;
                    node1 = edges[node1 as usize];
                }
            }
            if node2 != -1 {
                if dist2[node2 as usize] != -1 {
                    node2 = -1;
                } else if dist1[node2 as usize] != -1 {
                    ans.1 = Some(node2)
                } else {
                    dist2[node2 as usize] = dist;
                    node2 = edges[node2 as usize];
                }
            }
            match ans {
                (Some(node1), Some(node2)) => return node1.min(node2),
                (None, Some(node)) | (Some(node), None) => return node,
                (None, None) => dist += 1,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![2, 2, 3, -1], 0, 1, 2), (vec![1, 2, -1], 0, 2, 2)];
        for (edges, node1, node2, expected) in tt {
            let output = super::Solution::closest_meeting_node(edges, node1, node2);
            assert_eq!(expected, output);
        }
    }
}

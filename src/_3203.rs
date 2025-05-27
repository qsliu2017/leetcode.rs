#[allow(unused)]
struct Solution;
impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        fn build_nearby(edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
            let n = edges.len() + 1;
            let mut nearby = vec![vec![]; n];
            for edge in edges {
                let [u, v] = edge[..2] else { unreachable!() };
                let (u, v) = (u as usize, v as usize);
                nearby[u].push(v);
                nearby[v].push(u);
            }
            nearby
        }
        let nearby1 = build_nearby(&edges1);
        let nearby2 = build_nearby(&edges2);

        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        fn height(
            nearby: &Vec<Vec<usize>>,
            parent: usize,
            current: usize,
            max_diameter: &mut i32,
        ) -> i32 {
            let (h1, h2) = nearby[current]
                .iter()
                .filter(|&&child| child != parent)
                .map(|&child| height(nearby, current, child, max_diameter) + 1)
                .fold((0, 0), |(h1, h2), h| {
                    if h > h1 {
                        (h, h1)
                    } else if h > h2 {
                        (h1, h)
                    } else {
                        (h1, h2)
                    }
                });
            *max_diameter = (*max_diameter).max(h1 + h2);
            h1
        }

        let mut d1 = 0;
        let mut d2 = 0;
        height(&nearby1, n, 0, &mut d1);
        height(&nearby2, m, 0, &mut d2);

        d1.max(d2).max((d1 + 1) / 2 + (d2 + 1) / 2 + 1)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![0, 1], vec![0, 2], vec![0, 3]],
                vec![vec![0, 1]],
                3,
            ),
            (
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 4],
                    vec![2, 5],
                    vec![3, 6],
                    vec![2, 7],
                ],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 4],
                    vec![2, 5],
                    vec![3, 6],
                    vec![2, 7],
                ],
                5,
            ),
        ];
        for (edges1, edges2, expected) in tt {
            let output = super::Solution::minimum_diameter_after_merge(edges1, edges2);
            assert_eq!(expected, output);
        }
    }
}

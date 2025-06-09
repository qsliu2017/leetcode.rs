#[allow(unused)]
struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        let mut height_map = height_map;
        let mut pq = (0..n)
            .flat_map(|j| [(0, j), (m - 1, j)])
            .chain((1..m - 1).flat_map(|i| [(i, 0), (i, n - 1)]))
            .map(|(i, j)| {
                let h = height_map[i][j];
                height_map[i][j] = -1;
                Reverse((h, i, j))
            })
            .collect::<BinaryHeap<_>>();
        let mut ans = 0;
        while let Some(Reverse((height, i, j))) = pq.pop() {
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|&(dx, dy)| (i as i32 + dx, j as i32 + dy))
                .filter_map(|(x, y)| {
                    (0 <= x && x < m as i32 && 0 <= y && y < n as i32)
                        .then_some((x as usize, y as usize))
                })
                .for_each(|(x, y)| {
                    let h = height_map[x][y];
                    if h < 0 {
                        return;
                    }
                    height_map[x][y] = -1;
                    ans += (height - h).max(0);
                    pq.push(Reverse((height.max(h), x, y)));
                });
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
                vec![
                    vec![1, 4, 3, 1, 3, 2],
                    vec![3, 2, 1, 3, 2, 4],
                    vec![2, 3, 3, 2, 3, 1],
                ],
                4,
            ),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 2, 2, 2, 3],
                    vec![3, 2, 1, 2, 3],
                    vec![3, 2, 2, 2, 3],
                    vec![3, 3, 3, 3, 3],
                ],
                10,
            ),
        ];
        for (height_map, expected) in tt {
            let output = super::Solution::trap_rain_water(height_map);
            assert_eq!(expected, output);
        }
    }
}

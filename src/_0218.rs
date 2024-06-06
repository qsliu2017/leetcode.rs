struct Solution;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = buildings
            .iter()
            .flat_map(|b| [b[0] as usize, b[1] as usize])
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        points.sort();
        let (point2index, index2point) = points.into_iter().enumerate().fold(
            (HashMap::new(), HashMap::new()),
            |(mut p2i, mut i2p), (i, n)| {
                p2i.insert(n, i);
                i2p.insert(i, n);
                (p2i, i2p)
            },
        );
        buildings
            .into_iter()
            .fold(vec![0; point2index.len()], |mut heights, b| {
                let (l, r, h) = (b[0] as usize, b[1] as usize, b[2] as usize);
                let (l, r) = (point2index[&l], point2index[&r]);
                (l..r).for_each(|i| heights[i] = heights[i].max(h));
                heights
            })
            .into_iter()
            .enumerate()
            .fold((vec![], 0), |(mut skyline, prev), (i, h)| {
                if i == 0 || h != prev {
                    skyline.push(vec![index2point[&i] as i32, h as i32]);
                }
                (skyline, h)
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![0, 2, 3], vec![2, 5, 3]],
                vec![vec![0, 3], vec![5, 0]],
            ),
            (
                vec![
                    vec![2, 9, 10],
                    vec![3, 7, 15],
                    vec![5, 12, 12],
                    vec![15, 20, 10],
                    vec![19, 24, 8],
                ],
                vec![
                    vec![2, 10],
                    vec![3, 15],
                    vec![7, 12],
                    vec![12, 0],
                    vec![15, 10],
                    vec![20, 8],
                    vec![24, 0],
                ],
            ),
        ];
        for (input, expected) in tt {
            let output = super::Solution::get_skyline(input);
            assert_eq!(expected, output);
        }
    }
}

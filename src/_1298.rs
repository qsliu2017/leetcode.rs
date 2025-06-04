#[allow(unused)]
struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut keys_to_use = status
            .into_iter()
            .enumerate()
            .filter_map(|(box_id, s)| (s == 1).then_some(box_id as i32))
            .collect::<HashSet<_>>();
        let mut boxes_to_open = initial_boxes.into_iter().collect::<HashSet<_>>();
        let mut candies_cnt = 0;
        loop {
            let boxes = keys_to_use
                .intersection(&boxes_to_open)
                .copied()
                .collect::<Vec<_>>();
            if boxes.is_empty() {
                break;
            }
            boxes.into_iter().for_each(|box_id| {
                keys_to_use.extend(&keys[box_id as usize]);
                boxes_to_open.remove(&box_id);
                boxes_to_open.extend(&contained_boxes[box_id as usize]);
                candies_cnt += candies[box_id as usize];
            });
        }
        candies_cnt
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 100],
                vec![vec![], vec![], vec![1], vec![]],
                vec![vec![1, 2], vec![3], vec![], vec![]],
                vec![0],
                16,
            ),
            (
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1],
                vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
                vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
                vec![0],
                6,
            ),
            (
                vec![1, 1, 1],
                vec![100, 1, 100],
                vec![vec![], vec![0, 2], vec![]],
                vec![vec![], vec![], vec![]],
                vec![1],
                1,
            ),
            (vec![1], vec![100], vec![vec![]], vec![vec![]], vec![], 0),
            (
                vec![1, 1, 1],
                vec![2, 3, 2],
                vec![vec![], vec![], vec![]],
                vec![vec![], vec![], vec![]],
                vec![2, 1, 0],
                7,
            ),
        ];
        for (status, candies, keys, contained_boxes, initial_boxes, expected) in tt {
            let output =
                super::Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes);
            assert_eq!(expected, output);
        }
    }
}

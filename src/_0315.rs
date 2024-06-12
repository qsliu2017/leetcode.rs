#[allow(unused)]
struct Solution;
use std::collections::VecDeque;
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.into_iter()
            .rev()
            .fold(
                (VecDeque::with_capacity(n), Vec::with_capacity(n)),
                |(mut res, mut ordered), num| {
                    let i = ordered.partition_point(|&x| x < num);
                    ordered.insert(i, num);
                    res.push_front(i as i32);
                    (res, ordered)
                },
            )
            .0
            .into()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![5, 2, 6, 1], vec![2, 1, 1, 0]),
            (vec![-1], vec![0]),
            (vec![-1, -1], vec![0, 0]),
        ];
        for (input, expected) in tt {
            let output = super::Solution::count_smaller(input);
            assert_eq!(expected, output);
        }
    }
}

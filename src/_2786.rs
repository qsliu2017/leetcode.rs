#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let x = x as i64;
        nums.into_iter()
            .map(|n| n as i64)
            .fold(
                (0, None::<i64>, None::<i64>),
                |(max_score, even_max, odd_max), num| {
                    let is_odd = num % 2 == 1;
                    let from_even = even_max.map(|s| s - if is_odd { x } else { 0 });
                    let from_odd = odd_max.map(|s| s - if !is_odd { x } else { 0 });
                    let score = num
                        + match (from_even, from_odd) {
                            (None, None) => 0,
                            (None, Some(s)) | (Some(s), None) => s,
                            (Some(a), Some(b)) => a.max(b),
                        };
                    (
                        max_score.max(score),
                        if is_odd {
                            even_max
                        } else {
                            even_max.map(|s| s.max(score)).or(Some(score))
                        },
                        if !is_odd {
                            odd_max
                        } else {
                            odd_max.map(|s| s.max(score)).or(Some(score))
                        },
                    )
                },
            )
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![2, 3, 6, 1, 9, 2], 5, 13), (vec![2, 4, 6, 8], 3, 20)];
        for (nums, x, expected) in tt {
            let output = super::Solution::max_score(nums, x);
            assert_eq!(expected, output);
        }
    }
}

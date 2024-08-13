#[allow(unused)]
struct Solution;
use std::iter::from_fn;
impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        nums.into_iter()
            .fold(vec![vec![0; 10]; 9], |count, mut n| {
                // count[bit][number] = count
                from_fn(|| {
                    (n > 0).then(|| {
                        let ret = n % 10;
                        n /= 10;
                        ret
                    })
                })
                .enumerate()
                .fold(count, |mut count, (bit, number)| {
                    count[bit][number as usize] += 1;
                    count
                })
            })
            .into_iter()
            .map(|count| {
                count
                    .into_iter()
                    .fold((0, 0), |(sum, acc), c| (sum + acc * c, acc + c))
                    .0
            })
            .sum()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![13, 23, 12], 4), (vec![10, 10, 10, 10], 0)];
        for (nums, expected) in tt {
            let output = super::Solution::sum_digit_differences(nums);
            assert_eq!(expected, output);
        }
    }
}

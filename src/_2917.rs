#[allow(unused)]
struct Solution;
impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter()
            .fold([0; 32], |mut count, mut n| {
                let mut i = 0;
                while n > 0 {
                    count[i] += n & 1;
                    i += 1;
                    n >>= 1;
                }
                count
            })
            .into_iter()
            .rev()
            .reduce(|acc, count| (acc << 1) | (count >= k) as i32)
            .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![7, 12, 9, 8, 9, 15], 4, 9),
            (vec![2, 12, 1, 11, 4, 5], 6, 0),
            (vec![10, 8, 5, 9, 11, 6, 8], 1, 15),
        ];
        for (nums, k, expected) in tt {
            let output = super::Solution::find_k_or(nums, k);
            assert_eq!(expected, output);
        }
    }
}

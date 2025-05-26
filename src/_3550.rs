#[allow(unused)]
struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|mut num| {
                let mut acc = 0;
                while num > 0 {
                    acc += num % 10;
                    num /= 10;
                }
                acc
            })
            .enumerate()
            .find_map(|(i, sum)| (i as i32 == sum).then_some(i as i32))
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 3, 2], 2),
            (vec![1, 10, 11], 1),
            (vec![1, 2, 3], -1),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::smallest_index(nums);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut freq = [0; 101];
        let start = nums
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &n)| {
                let n = n as usize;
                freq[n] += 1;
                freq[n] > 1
            })
            .map_or(-1, |(i, _)| i as i32);
        (start + 3) / 3
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 2, 3, 4, 2, 3, 3, 5, 7], 2),
            (vec![4, 5, 6, 4, 4], 2),
            (vec![6, 7, 8, 9], 0),
        ];
        for (nums, expected) in tt {
            let output = super::Solution::minimum_operations(nums);
            assert_eq!(expected, output);
        }
    }
}

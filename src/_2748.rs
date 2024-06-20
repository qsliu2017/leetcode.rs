#[allow(unused)]
struct Solution;
impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let primes = [
            vec![],
            /* 1 */ vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            /* 2 */ vec![1, 3, 5, 7, 9],
            /* 3 */ vec![1, 2, 4, 5, 7, 8],
            /* 4 */ vec![1, 3, 5, 7, 9],
            /* 5 */ vec![1, 2, 3, 4, 6, 7, 8, 9],
            /* 6 */ vec![1, 5, 7],
            /* 7 */ vec![1, 2, 3, 4, 5, 6, 8, 9],
            /* 8 */ vec![1, 3, 5, 7, 9],
            /* 9 */ vec![1, 2, 4, 5, 7, 8],
        ];
        nums.into_iter()
            .map(|n| {
                let last = n % 10;
                let mut first = n;
                while first >= 10 {
                    first /= 10;
                }
                (first as usize, last as usize)
            })
            .fold((0, vec![0; 10]), |(mut ans, mut count), (first, last)| {
                ans += primes[last].iter().map(|&i| count[i]).sum::<i32>();
                count[first] += 1;
                (ans, count)
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![2, 5, 1, 4], 5),
            (vec![11, 21, 12], 2),
            (vec![35, 52, 74, 92, 25, 65, 77, 1, 73, 32], 37),
        ];
        for (input, expected) in tt {
            let output = super::Solution::count_beautiful_pairs(input);
            assert_eq!(expected, output);
        }
    }
}

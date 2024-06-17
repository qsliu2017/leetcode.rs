#[allow(unused)]
struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut upper = nums.iter().map(|&n| n as i64).sum::<i64>();
        if k == 1 {
            return upper as _;
        }
        let mut lower = *nums.iter().max().unwrap() as i64;
        while lower < upper {
            let s = (upper + lower) / 2;
            if nums
                .iter()
                .map(|&n| n as i64)
                .fold((0, 0), |(sum, k), n| {
                    if sum + n <= s {
                        (sum + n, k)
                    } else {
                        (n, k + 1)
                    }
                })
                .1
                < k
            {
                upper = s;
            } else {
                lower = s + 1;
            }
        }
        lower as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![7, 2, 5, 10, 8], 2, 18),
            (vec![1, 2, 3, 4, 5], 2, 9),
            (vec![1, 4, 4], 3, 4),
            (vec![1000000; 1000], 1, 1000000000),
        ];
        for (nums, k, expected) in tt {
            let output = super::Solution::split_array(nums, k);
            assert_eq!(expected, output);
        }
    }
}

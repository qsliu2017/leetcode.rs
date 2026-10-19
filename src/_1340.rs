#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        fn max_jumps_from(arr: &Vec<i32>, dp: &mut Vec<usize>, d: usize, i: usize) -> usize {
            let left = (if i >= d { i - d } else { 0 }..i)
                .rev()
                .map_while(|j| {
                    (arr[j] < arr[i]).then(|| {
                        if dp[j] == 0 {
                            max_jumps_from(arr, dp, d, j)
                        } else {
                            dp[j]
                        }
                    })
                })
                .max()
                .unwrap_or(0);
            let right = (i + 1..(i + d + 1).min(arr.len()))
                .map_while(|j| {
                    (arr[j] < arr[i]).then(|| {
                        if dp[j] == 0 {
                            max_jumps_from(arr, dp, d, j)
                        } else {
                            dp[j]
                        }
                    })
                })
                .max()
                .unwrap_or(0);
            dp[i] = left.max(right) + 1;
            dp[i]
        }
        let mut dp = vec![0; arr.len()];
        (0..arr.len())
            .map(|i| max_jumps_from(&arr, &mut dp, d, i))
            .max()
            .unwrap() as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2, 4),
            (vec![3, 3, 3, 3, 3], 3, 1),
            (vec![7, 6, 5, 4, 3, 2, 1], 1, 7),
            (vec![7, 1, 7, 1, 7, 1], 2, 2),
            (vec![66], 1, 1),
        ];
        for (arr, d, expected) in tt {
            let output = super::Solution::max_jumps(arr, d);
            assert_eq!(expected, output);
        }
    }
}

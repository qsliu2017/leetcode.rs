#[allow(unused)]
struct Solution;
impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        if p == 0 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let p = p as usize;
        let mut lower = 0;
        let mut upper = nums.windows(2).map(|w| w[1] - w[0]).max().unwrap();
        while lower != upper {
            let mid = (lower + upper) / 2;
            if nums
                .windows(2)
                .map(|w| w[1] - w[0])
                .scan((0, 0), |(dp0, dp1), diff| {
                    if diff <= mid {
                        *dp0 += 1;
                    }
                    *dp0 = *dp0.max(dp1);
                    std::mem::swap(dp0, dp1);
                    Some(*dp1)
                })
                .find(|&pairs| pairs >= p)
                .is_none()
            {
                lower = mid + 1;
            } else {
                upper = mid;
            }
        }
        lower
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![10, 1, 2, 7, 1, 3], 2, 1), (vec![4, 2, 1, 2], 1, 0)];
        for (nums, p, expected) in tt {
            let output = super::Solution::minimize_max(nums, p);
            assert_eq!(expected, output);
        }
    }
}

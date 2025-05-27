#[allow(unused)]
struct Solution;
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let n = jobs.len();
        let n_state = 2_usize.pow(n as _);
        let time_required = |state: usize| {
            jobs.iter()
                .enumerate()
                .filter_map(|(i, job)| ((state >> i) & 1 != 0).then_some(job))
                .sum::<i32>()
        };
        let time_required = (1..k).fold(
            (0..n_state).map(time_required).collect::<Vec<_>>(),
            |dp, _| {
                (0..n_state)
                    .map(|final_state| {
                        (0..n_state)
                            .filter(|&prev_state| (prev_state & final_state) == prev_state)
                            .map(|prev_state| {
                                time_required(prev_state ^ final_state).max(dp[prev_state])
                            })
                            .min()
                            .unwrap_or(0)
                    })
                    .collect()
            },
        );
        time_required[n_state - 1]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![3, 2, 3], 3, 3), (vec![1, 2, 4, 7, 8], 2, 11)];
        for (jobs, k, expected) in tt {
            let output = super::Solution::minimum_time_required(jobs, k);
            assert_eq!(expected, output);
        }
    }
}

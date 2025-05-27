#[allow(unused)]
struct Solution;
impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let n = tasks.len();
        let n_states = 1 << n;
        let done_at_once = |state| {
            tasks
                .iter()
                .enumerate()
                .map(|(i, time)| (state >> i & 1) as i32 * time)
                .sum::<i32>()
                <= session_time
        };
        let min_sessions = (1..n_states).fold(vec![0; n_states], |mut dp, state| {
            dp[state] = if done_at_once(state) {
                1
            } else {
                let mut sub_state = state;
                std::iter::from_fn(|| {
                    sub_state = (sub_state - 1) & state;
                    (sub_state > 0).then_some(sub_state)
                })
                .map(|left_state| dp[left_state] + dp[left_state ^ state])
                .min()
                .unwrap()
            };
            dp
        });
        min_sessions[n_states - 1]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 2, 3], 3, 2),
            (vec![3, 1, 3, 1, 1], 8, 2),
            (vec![1, 2, 3, 4, 5], 15, 1),
        ];
        for (tasks, session_time, expected) in tt {
            let output = super::Solution::min_sessions(tasks, session_time);
            assert_eq!(expected, output);
        }
    }
}

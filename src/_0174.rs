#[allow(unused)]
struct Solution;
use std::iter;
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let n = dungeon[0].len();
        // (min_hp, current_hp) order by min_hp desc
        let mut dp = vec![vec![]; n];
        dp[0].push((0, 0));
        for row in dungeon {
            for (i, cell) in row.into_iter().enumerate() {
                let mut right = 0;
                let mut up = 0;
                let mut max_current_hp = i32::MIN;
                dp[i] = iter::from_fn(|| {
                    while up < dp[i].len() && (i > 0 && right < dp[i - 1].len()) {
                        if dp[i][up] > dp[i - 1][right] {
                            up += 1;
                            return Some(dp[i][up - 1]);
                        } else {
                            right += 1;
                            return Some(dp[i - 1][right - 1]);
                        }
                    }
                    while up < dp[i].len() {
                        up += 1;
                        return Some(dp[i][up - 1]);
                    }
                    while i > 0 && right < dp[i - 1].len() {
                        right += 1;
                        return Some(dp[i - 1][right - 1]);
                    }
                    None
                })
                .filter_map(|(min_hp, current_hp)| {
                    (current_hp > max_current_hp).then(|| {
                        max_current_hp = current_hp;
                        let current_hp = current_hp + cell;
                        (min_hp.min(current_hp), current_hp)
                    })
                })
                .collect();
            }
        }
        (1 - dp[n - 1].iter().map(|(min_hp, _)| min_hp).max().unwrap()).max(1)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]], 7),
            (vec![vec![0]], 1),
        ];
        for (dungeon, expected) in tt {
            let output = super::Solution::calculate_minimum_hp(dungeon);
            assert_eq!(expected, output);
        }
    }
}

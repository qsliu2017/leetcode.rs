#[allow(unused)]
struct Solution;
fn pow_ring(mut base: i64, mut exp: u32, mod_: i64) -> i64 {
    let mut ans = 1;
    while exp > 0 {
        base %= mod_;
        if exp & 1 == 1 {
            ans *= base;
            ans %= mod_;
        }
        base *= base;
        exp >>= 1;
    }
    ans
}
impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        variables
            .into_iter()
            .map(|v| (v[0] as i64, v[1] as u32, v[2] as u32, v[3] as i64))
            .map(|(a, b, c, m)| pow_ring(pow_ring(a, b, 10), c, m))
            .enumerate()
            .filter_map(|(i, v)| (v == target as i64).then_some(i as i32))
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use crate::_2961::pow_ring;
    #[test]
    fn test_pow_ring() {
        assert_eq!(pow_ring(1, 3, 10), 1);
        assert_eq!(pow_ring(pow_ring(1, 3, 10), 3, 2), 1);
    }
    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]],
                2,
                vec![0, 2],
            ),
            (vec![vec![39, 3, 1000, 1000]], 17, vec![]),
        ];
        for (variables, target, expected) in tt {
            let output = super::Solution::get_good_indices(variables, target);
            assert_eq!(expected, output);
        }
    }
}

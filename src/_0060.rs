#[allow(unused)]
struct Solution;
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n: usize = n as usize;
        let k = k as usize - 1;
        (1..=n)
            .into_iter()
            .rev()
            .fold(
                (
                    Vec::with_capacity(n),
                    k,
                    (1..=n).into_iter().product::<usize>(),
                    Vec::from_iter(1..=n),
                ),
                |(mut ans, k, pow, mut nums), i| {
                    let pow = pow / i;
                    let ith = k / pow;
                    let num = nums.remove(ith);
                    ans.push(num);
                    (ans, k - ith * pow, pow, nums)
                },
            )
            .0
            .into_iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(3, 3, "213"), (4, 9, "2314"), (3, 1, "123")];
        for (n, k, expected) in tt {
            let output = super::Solution::get_permutation(n, k);
            assert_eq!(expected.to_string(), output);
        }
    }
}

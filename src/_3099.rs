#[allow(unused)]
struct Solution;
impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let sum = x.to_string().bytes().map(|b| (b - b'0') as i32).sum();
        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(18, 9), (23, -1)];
        for (x, expected) in tt {
            let output = super::Solution::sum_of_the_digits_of_harshad_number(x);
            assert_eq!(expected, output);
        }
    }
}

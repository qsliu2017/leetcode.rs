#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        let num = num as usize;
        let sum = sum as usize;
        if sum > num * 9 {
            return "".to_string();
        }

        let nines = sum / 9;
        let mut ans = vec![0_u8; num];
        ans[0..nines].fill(b'9');
        if nines < num {
            ans[nines] = b'0' + (sum % 9) as u8;
            ans[nines + 1..].fill(b'0');
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(2, 3, "30"), (2, 17, "98"), (1, 10, "")];
        for (num, sum, expected) in tt {
            let output = super::Solution::max_sum_of_squares(num, sum);
            assert_eq!(expected, output);
        }
    }
}

struct Solution;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let mut a = 0; // decode # of &s[0..i-1]
        let mut b = 1; // decode # of &s[0..i]
        for i in 0..n {
            let mut c = 0;
            let n = s[i..i + 1].parse::<i32>().unwrap();
            if n > 0 {
                c += b;
            }
            if i > 0 {
                let n = s[i - 1..i + 1].parse::<i32>().unwrap();
                if n >= 10 && n <= 26 {
                    c += a;
                }
            }
            (a, b) = (b, c);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::_0091::Solution;

    #[test]
    fn test() {
        let tt = [("12", 2), ("226", 3), ("06", 0)];
        for (input, output) in tt {
            assert_eq!(Solution::num_decodings(input.to_string()), output);
        }
    }
}

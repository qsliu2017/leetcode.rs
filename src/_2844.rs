#[allow(unused)]
struct Solution;
impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let mut it = num.as_bytes().iter().rev().enumerate();
        let (mut last_0, mut last_5) = (usize::MAX, usize::MAX);
        while let Some((i, &n)) = it.next() {
            match (n, last_0) {
                (b'0', usize::MAX) => last_0 = i,
                (b'0' | b'5', _) if last_0 != usize::MAX => return i as i32 - 1,
                _ => {}
            }
            match (n, last_5) {
                (b'5', usize::MAX) => last_5 = i,
                (b'2' | b'7', _) if last_5 != usize::MAX => return i as i32 - 1,
                _ => {}
            }
        }
        num.len() as i32 - if last_0 == usize::MAX { 0 } else { 1 }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [("2245047", 2), ("2908305", 3), ("10", 1)];
        for (num, expected) in tt {
            let output = super::Solution::minimum_operations(num.to_string());
            assert_eq!(expected, output);
        }
    }
}

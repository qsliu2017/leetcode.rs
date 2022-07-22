struct Solution;
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0, 1];
        let mut p = 1;
        for _ in 1..n {
            p *= 2;
            let mut append = res.iter().rev().map(|&e| e + p).collect();
            res.append(&mut append);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use crate::_0089::Solution;

    #[test]
    fn test() {
        let tt = [(2, vec![0, 1, 3, 2]), (1, vec![0, 1])];
        for (input, output) in tt {
            assert_eq!(Solution::gray_code(input), output);
        }
    }
}

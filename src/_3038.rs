#[allow(unused)]
struct Solution;
impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut iter = nums.chunks_exact(2).map(|chk| chk[0] + chk[1]);
        iter.next()
            .map(|first| iter.take_while(|&n| n == first).count() + 1)
            .unwrap_or(0) as _
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [(vec![3, 2, 1, 4, 5], 2)];
        for (input, expected) in tt {
            let output = super::Solution::max_operations(input);
            assert_eq!(expected, output);
        }
    }
}

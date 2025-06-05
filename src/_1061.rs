#[allow(unused)]
struct Solution;
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut union_set = Vec::from_iter(0..26);
        fn find(union_set: &mut Vec<usize>, i: usize) -> usize {
            let parent = union_set[i];
            if parent == i {
                parent
            } else {
                let root = find(union_set, parent);
                union_set[i] = root;
                root
            }
        };
        let mut union = |i: usize, j: usize| {
            let x = find(&mut union_set, i);
            let y = find(&mut union_set, j);
            if x == y {
                false
            } else {
                let (x, y) = (x.min(y), x.max(y));
                union_set[y] = x;
                true
            }
        };
        s1.bytes()
            .zip(s2.bytes())
            .map(|(a, b)| (a - b'a', b - b'a'))
            .for_each(|(a, b)| {
                union(a as usize, b as usize);
            });
        String::from_utf8(
            base_str
                .bytes()
                .map(|b| find(&mut union_set, (b - b'a') as usize) as u8 + b'a')
                .collect(),
        )
        .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            ("parker", "morris", "parser", "makkek"),
            ("hello", "world", "hold", "hdld"),
            ("leetcode", "programs", "sourcecode", "aauaaaaada"),
        ];
        for (s1, s2, base_str, expected) in tt {
            let output = super::Solution::smallest_equivalent_string(
                s1.to_string(),
                s2.to_string(),
                base_str.to_string(),
            );
            assert_eq!(expected, output);
        }
    }
}

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // Self::count_windows(s, t)
        Self::double_pointer(s, t)
    }

    fn double_pointer(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let (mut from, mut to) = (0, usize::MAX);
        let mut from_ptr = 0;
        let count: std::collections::HashMap<_, usize> =
            t.iter()
                .fold(std::collections::HashMap::new(), |mut cnt, &c| {
                    let n = cnt.get(&c).and_then(|&v| Some(v)).unwrap_or(0) + 1;
                    cnt.insert(c, n);
                    cnt
                });
        let need = count.iter().count();
        let mut have = 0;
        let mut current_count = std::collections::HashMap::new();

        for to_ptr in 0..s.len() {
            let n = current_count
                .get(&s[to_ptr])
                .and_then(|&v| Some(v))
                .unwrap_or(0)
                + 1;
            current_count.insert(s[to_ptr], n);
            if count.get(&s[to_ptr]).and_then(|&v| Some(v)).unwrap_or(0) == n {
                have += 1;
            }
            while have == need {
                if to_ptr - from_ptr + 1 < to - from {
                    // (from, to) = (from_ptr, to_ptr + 1);
                    from = from_ptr;
                    to = to_ptr + 1;
                }
                let n = current_count
                    .get(&s[from_ptr])
                    .and_then(|&v| Some(v))
                    .unwrap()
                    - 1;
                current_count.insert(s[from_ptr], n);
                if count.get(&s[from_ptr]).and_then(|&v| Some(v)).unwrap_or(0) > n {
                    have -= 1;
                }
                from_ptr += 1;
            }
        }

        if to < usize::MAX {
            return String::from_utf8(s[from..to].to_vec()).unwrap();
        }

        String::from("")
    }

    fn count_windows(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::from("");
        }

        let count: std::collections::HashMap<char, i32> =
            t.chars()
                .fold(std::collections::HashMap::new(), |mut cnt, c| {
                    let n = cnt.get(&c).and_then(|&v| Some(v)).unwrap_or(0) + 1;
                    cnt.insert(c, n);
                    cnt
                });

        let mut window_counts = (0..s.len() - t.len() + 1)
            .map(|start| {
                s[start..start + t.len() - 1].chars().fold(
                    std::collections::HashMap::new(),
                    |mut cnt, c| {
                        let n = cnt.get(&c).and_then(|&v| Some(v)).unwrap_or(0) + 1;
                        cnt.insert(c, n);
                        cnt
                    },
                )
            })
            .collect::<Vec<_>>();

        for length in t.len()..s.len() + 1 {
            for start in 0..s.len() - length + 1 {
                let window_count = &mut window_counts[start];
                let c = s.chars().nth(start + length - 1).unwrap();
                let n = window_count.get(&c).and_then(|&v| Some(v)).unwrap_or(0) + 1;
                window_count.insert(c, n);

                if count.iter().all(|(k, v)| {
                    let cnt = window_count.get(k).unwrap_or(&0);
                    cnt >= v
                }) {
                    return s[start..start + length].to_string();
                }
            }
        }

        String::from("")
    }
}
#[cfg(test)]
mod tests {
    use crate::_0076::Solution;

    #[test]
    fn test() {
        let tt = [
            ("ADOBECODEBANC", "ABC", "BANC"),
            ("a", "a", "a"),
            ("a", "aa", ""),
        ];
        for (s, t, output) in tt {
            assert_eq!(
                Solution::min_window(s.to_string(), t.to_string()),
                output.to_string()
            );
        }
    }
}

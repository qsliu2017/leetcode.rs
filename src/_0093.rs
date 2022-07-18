struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let n = s.len();
        let mut res = vec![];

        // choose (i, j, k) to split s into four parts: [0, i), [i, j), [j, k) and [k, n).
        for i in 0 + 1..n.min(0 + 4) {
            if !Self::is_part_vaild(&s[0..i]) {
                continue;
            }
            for j in i + 1..n.min(i + 4) {
                if !Self::is_part_vaild(&s[i..j]) {
                    continue;
                }
                for k in (n - 4).max(j + 1)..n.min(j + 4) {
                    if !Self::is_part_vaild(&s[j..k]) || !Self::is_part_vaild(&s[k..n]) {
                        continue;
                    }
                    res.push(format!(
                        "{}.{}.{}.{}",
                        &s[0..i],
                        &s[i..j],
                        &s[j..k],
                        &s[k..n]
                    ));
                }
            }
        }
        res
    }

    fn is_part_vaild(s: &str) -> bool {
        (s.len() == 1 || s.chars().nth(0).unwrap() != '0') && s.parse::<i32>().unwrap() <= 255
    }
}
#[cfg(test)]
mod tests {
    use crate::_0093::Solution;

    #[test]
    fn test() {
        dbg!(Solution::restore_ip_addresses("25525511135".to_string()));
        //     let tt = [
        //         ("25525511135", vec!["255.255.11.135", "255.255.111.35"]),
        //         ("0000", vec!["0.0.0.0"]),
        //         (
        //             "101023",
        //             vec![
        //                 "1.0.10.23",
        //                 "1.0.102.3",
        //                 "10.1.0.23",
        //                 "10.10.2.3",
        //                 "101.0.2.3",
        //             ],
        //         ),
        //     ];
        //     for (input, output) in tt {
        //         assert_eq!(
        //             Solution::restore_ip_addresses(input.to_string()),
        //             output.try_into().unwrap()
        //         );
        //     }
    }
}

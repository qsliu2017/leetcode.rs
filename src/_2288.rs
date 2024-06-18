#[allow(unused)]
struct Solution;
impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        sentence
            .split(' ')
            .into_iter()
            .map(|word| {
                let bytes = word.as_bytes();
                if bytes[0] == b'$'
                    && bytes.len() > 1
                    && bytes[1..].iter().all(|b| b.is_ascii_digit())
                {
                    let price = bytes[1..]
                        .iter()
                        .fold(0, |acc, &digit| acc * 10 + (digit - b'0') as i64);
                    let price = (price * (100 - discount as i64)) as f64 * 0.01;
                    format!("${:.2}", price)
                } else {
                    word.to_string()
                }
            })
            .reduce(|sentence, word| sentence + " " + &word)
            .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                "there are $1 $2 and 5$ candies in the shop",
                50,
                "there are $0.50 $1.00 and 5$ candies in the shop",
            ),
            (
                "1 2 $3 4 $5 $6 7 8$ $9 $10$",
                100,
                "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$",
            ),
        ];
        for (sentence, discount, expected) in tt {
            let output = super::Solution::discount_prices(sentence.to_string(), discount);
            assert_eq!(expected, output.to_string());
        }
    }
}

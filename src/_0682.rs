#[allow(unused)]
struct Solution;
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        operations
            .into_iter()
            .fold(vec![], |mut stack, op| {
                let n = stack.len();
                match op.as_str() {
                    "C" => {
                        stack.pop();
                    }
                    "D" => {
                        stack.push(stack[n - 1] * 2);
                    }
                    "+" => {
                        stack.push(stack[n - 1] + stack[n - 2]);
                    }
                    op => {
                        stack.push(i32::from_str_radix(op, 10).unwrap());
                    }
                };
                stack
            })
            .into_iter()
            .sum()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec!["5", "2", "C", "D", "+"], 30),
            (vec!["5", "-2", "4", "C", "D", "9", "+", "+"], 27),
            (vec!["1"], 1),
        ];
        for (operations, expected) in tt {
            let output = super::Solution::cal_points(
                operations.into_iter().map(|s| s.to_string()).collect(),
            );
            assert_eq!(expected, output);
        }
    }
}

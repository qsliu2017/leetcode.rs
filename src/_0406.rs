#[allow(unused)]
struct Solution;
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        let n = people.len();
        people.sort_by(|i, j| j[0].cmp(&i[0]).then(i[1].cmp(&j[1])));
        people
            .into_iter()
            .fold(Vec::with_capacity(n), |mut queue, p| {
                queue.insert(p[1] as usize, p);
                queue
            })
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![
                    vec![7, 0],
                    vec![4, 4],
                    vec![7, 1],
                    vec![5, 0],
                    vec![6, 1],
                    vec![5, 2],
                ],
                vec![
                    vec![5, 0],
                    vec![7, 0],
                    vec![5, 2],
                    vec![6, 1],
                    vec![4, 4],
                    vec![7, 1],
                ],
            ),
            (
                vec![
                    vec![6, 0],
                    vec![5, 0],
                    vec![4, 0],
                    vec![3, 2],
                    vec![2, 2],
                    vec![1, 4],
                ],
                vec![
                    vec![4, 0],
                    vec![5, 0],
                    vec![2, 2],
                    vec![3, 2],
                    vec![1, 4],
                    vec![6, 0],
                ],
            ),
        ];
        for (input, expected) in tt {
            let output = super::Solution::reconstruct_queue(input);
            assert_eq!(expected, output);
        }
    }
}

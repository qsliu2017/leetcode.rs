struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let (mut degree, adjacent) = prerequisites.into_iter().fold(
            (vec![0; num_courses], vec![vec![]; num_courses]),
            |(mut degree, mut adjacent), prereq| {
                let (i, j) = (prereq[0] as usize, prereq[1] as usize);
                adjacent[j].push(i);
                degree[i] += 1;
                (degree, adjacent)
            },
        );

        let mut to_visit = degree
            .iter()
            .enumerate()
            .filter(|(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        while let Some(next) = to_visit.pop() {
            adjacent[next].iter().for_each(|&i| {
                degree[i] -= 1;
                if degree[i] == 0 {
                    to_visit.push(i);
                }
            });
        }

        degree.into_iter().all(|d| d == 0)
    }
}
#[cfg(test)]
mod tests {
    use crate::_0207::Solution;

    #[test]
    fn test() {
        let tt = [
            (2, vec![vec![1, 0]], true),
            (2, vec![vec![1, 0], vec![0, 1]], false),
            (
                5,
                vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]],
                true,
            ),
        ];
        for (num_courses, prerequisites, output) in tt {
            assert_eq!(Solution::can_finish(num_courses, prerequisites), output);
        }
    }
}

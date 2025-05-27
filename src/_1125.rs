#[allow(unused)]
struct Solution;
impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let req_skills = req_skills
            .into_iter()
            .enumerate()
            .map(|(i, skill)| (skill, i))
            .collect::<std::collections::HashMap<_, _>>();
        let people = people
            .into_iter()
            .map(|skills| {
                skills
                    .into_iter()
                    .map(|skill| req_skills.get(&skill).unwrap())
                    .map(|&i| 1_usize << i)
                    .fold(0, |acc, a| acc | a)
            })
            .collect::<Vec<_>>();
        let n_states = 1 << req_skills.len();

        let mut min_team_size = vec![i32::MAX; n_states];
        min_team_size[0] = 0;
        let mut prev_state = vec![0; n_states];
        let mut prev_people = vec![people.len(); n_states];

        for (i, &skills) in people.iter().enumerate() {
            for state in 0..n_states {
                let next_state = state | skills;
                if min_team_size[next_state] - 1 > min_team_size[state] {
                    min_team_size[next_state] = min_team_size[state] + 1;
                    prev_state[next_state] = state;
                    prev_people[next_state] = i;
                }
            }
        }

        let mut state = n_states - 1;
        let mut team = std::iter::from_fn(move || {
            let ppl = prev_people[state];
            (ppl != people.len()).then(|| {
                state = prev_state[state];
                ppl as _
            })
        })
        .collect::<Vec<_>>();
        team.reverse();
        team
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec!["java", "nodejs", "reactjs"],
                vec![vec!["java"], vec!["nodejs"], vec!["nodejs", "reactjs"]],
                vec![0, 2],
            ),
            (
                vec!["algorithms", "math", "java", "reactjs", "csharp", "aws"],
                vec![
                    vec!["algorithms", "math", "java"],
                    vec!["algorithms", "math", "reactjs"],
                    vec!["java", "csharp", "aws"],
                    vec!["reactjs", "csharp"],
                    vec!["csharp", "math"],
                    vec!["aws", "java"],
                ],
                vec![1, 2],
            ),
        ];
        for (req_skills, people, expected) in tt {
            let output = super::Solution::smallest_sufficient_team(
                req_skills.into_iter().map(str::to_string).collect(),
                people
                    .into_iter()
                    .map(|skill| skill.into_iter().map(str::to_string).collect())
                    .collect(),
            );
            assert_eq!(expected, output);
        }
    }
}

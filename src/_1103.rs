struct Solution;
impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let (candies, num_people) = (candies as i64, num_people as i64);
        // find the largest number q s.t q(q+1)/2 < candies
        let q = ((-1.0 + ((1 + 8 * candies) as f64).sqrt()) / 2.0).floor() as i64;
        let round = q / num_people;
        let remain = q % num_people;
        let last = candies - (q * (q + 1) / 2);
        (0..num_people)
            .into_iter()
            .map(|i| {
                round * (i + 1)
                    + (round * (round - 1) / 2) * num_people
                    + if i < remain {
                        i + 1 + round * num_people
                    } else if i == remain {
                        last
                    } else {
                        0
                    }
            })
            .map(|i| i as i32)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (7, 4, vec![1, 2, 3, 1]),
            (10, 3, vec![5, 2, 3]),
            (90, 4, vec![27, 18, 21, 24]),
            (1000000000, 1000, vec![]),
        ];
        for (candies, num_people, expected) in tt {
            let output = super::Solution::distribute_candies(candies, num_people);
            assert_eq!(output, expected);
        }
    }
}

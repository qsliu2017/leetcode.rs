use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cnt = [0 as usize; 21];
        nums.iter().for_each(|&n| cnt[n as usize + 10] += 1);
        let cnt = dbg!(cnt);

        // subsets[i] is all subsets with card `i`
        let mut subsets = vec![vec![[0 as usize; 21]]];
        for card in 1..nums.len() + 1 {
            let mut all_subsets_of_card = HashSet::new();
            for subset in &subsets[card - 1] {
                (0..21).filter(|&i| cnt[i] - subset[i] > 0).for_each(|i| {
                    let mut subset = subset.clone();
                    subset[i] += 1;
                    all_subsets_of_card.insert(subset);
                });
            }
            subsets.push(all_subsets_of_card.into_iter().collect());
        }

        subsets
            .into_iter()
            .flat_map::<Vec<Vec<i32>>, _>(|subsets| {
                subsets
                    .into_iter()
                    .map(|cnt_of_each| {
                        let mut v = vec![];
                        for i in 0..21 {
                            v.append(&mut vec![i as i32 - 10; cnt_of_each[i]]);
                        }
                        v
                    })
                    .collect()
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use crate::_0090::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                vec![1, 2, 2],
                vec![
                    vec![],
                    vec![1],
                    vec![1, 2],
                    vec![1, 2, 2],
                    vec![2],
                    vec![2, 2],
                ],
            ),
            (vec![0], vec![vec![], vec![0]]),
        ];
        for (input, output) in tt {
            assert_eq!(Solution::subsets_with_dup(input), output);
        }
    }
}

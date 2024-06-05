struct Solution;
use std::cmp::Ordering;
impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (arr1, arr2, _, _) = nums.into_iter().enumerate().fold(
            (
                Vec::with_capacity(n),
                Vec::with_capacity(n),
                Vec::with_capacity(n),
                Vec::with_capacity(n),
            ),
            |(mut arr1, mut arr2, mut sorted1, mut sorted2), (i, n)| {
                let (arr, sorted) = match i {
                    0 => (&mut arr1, &mut sorted1),
                    1 => (&mut arr2, &mut sorted2),
                    _ => match sorted1
                        .partition_point(|&x| x > n)
                        .cmp(&sorted2.partition_point(|&x| x > n))
                    {
                        Ordering::Greater => (&mut arr1, &mut sorted1),
                        Ordering::Less => (&mut arr2, &mut sorted2),
                        Ordering::Equal => {
                            if arr1.len() <= arr2.len() {
                                (&mut arr1, &mut sorted1)
                            } else {
                                (&mut arr2, &mut sorted2)
                            }
                        }
                    },
                };
                arr.push(n);
                sorted.insert(sorted.partition_point(|&x| x > n), n);
                (arr1, arr2, sorted1, sorted2)
            },
        );
        arr1.into_iter().chain(arr2.into_iter()).collect()
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let tt = [
            (vec![2, 1, 3, 3], vec![2, 3, 1, 3]),
            (vec![5, 14, 3, 1, 2], vec![5, 3, 1, 2, 14]),
            (vec![3, 3, 3, 3], vec![3, 3, 3, 3]),
        ];
        for (input, expected) in tt {
            let output = super::Solution::result_array(input);
            assert_eq!(expected, output);
        }
    }
}

struct Solution;
impl Solution {
    fn cheat_method(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(vec![], |mut list, num| {
                if let Some(idx) = list.binary_search(&num).err() {
                    if idx < list.len() {
                        list[idx] = num;
                    } else {
                        list.push(num);
                    }
                }
                list
            })
            .len() as i32
    }
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::cheat_method(nums)
    }
    fn rcount_method(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .rfold(vec![], |next_v_len, num| {
                let len = next_v_len
                    .iter()
                    .fold(1, |max, (v, l)| if num < *v { max.max(l + 1) } else { max });

                next_v_len
                    .into_iter()
                    .filter_map(|(v, l)| {
                        if v <= num && l <= len {
                            None
                        } else {
                            Some((v, l))
                        }
                    })
                    .chain(std::iter::once((num, len)))
                    .collect()
            })
            .into_iter()
            .fold(0, |max, (_, l)| max.max(l))
    }
}
#[cfg(test)]
mod tests {
    use crate::_0300::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![10, 9, 2, 5, 3, 7, 101, 18], 4),
            (vec![0, 1, 0, 3, 2, 3], 4),
            (vec![7, 7, 7, 7, 7, 7, 7], 1),
        ];
        for (input, output) in tt {
            assert_eq!(Solution::length_of_lis(input), output);
        }
    }
}

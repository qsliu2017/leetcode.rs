#[allow(unused)]
struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        let (m, n) = (nums1.len(), nums2.len());
        let k = (m + n) / 2;

        let left = |nums: &Vec<i32>, index: usize| match index {
            0 => i32::MIN,
            _ => nums[index - 1],
        };
        let right = |nums: &Vec<i32>, index: usize| match index {
            _ if index >= nums.len() => i32::MAX,
            _ => nums[index],
        };

        // find a split i, and correspondingly j = k - i,
        // s.t. nums1[..i] and nums2[..j] are top-k.
        let i = match (0..=m.min(k)).collect::<Vec<_>>().binary_search_by(|&i| {
            let j = k - i;
            let left_max = i32::max(left(&nums1, i), left(&nums2, j));
            let right_min = i32::min(right(&nums1, i), right(&nums2, j));
            match left_max.cmp(&right_min) {
                std::cmp::Ordering::Equal | std::cmp::Ordering::Less => std::cmp::Ordering::Equal,
                std::cmp::Ordering::Greater => left(&nums1, i).cmp(&left(&nums2, j)),
            }
        }) {
            Ok(i) | Err(i) => i,
        };
        let j = k - i;
        if (m + n) % 2 == 1 {
            right(&nums1, i).min(right(&nums2, j)) as _
        } else {
            (left(&nums1, i).max(left(&nums2, j)) + right(&nums1, i).min(right(&nums2, j))) as f64
                / 2.0
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 3], vec![2], 2.0f64),
            (vec![1, 2], vec![3, 4], 2.5f64),
            (vec![2, 2, 4, 4], vec![2, 2, 2, 4, 4], 2.0),
            (vec![2, 3, 4, 5], vec![], 3.5),
        ];
        for (nums1, nums2, expected) in tt {
            let output = super::Solution::find_median_sorted_arrays(nums1, nums2);
            assert_eq!(expected, output);
        }
    }
}

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();
        nums1
            .windows(3)
            .zip(nums2)
            .map(|(w, n)| [n - w[0], n - w[1], n - w[2]])
            .reduce(|[a, b, c], [x, y, z]| {
                [
                    /* skip 0 */ (x == a).then_some(a).unwrap_or(i32::MAX),
                    /* skip 1 */
                    (y == a)
                        .then_some(a)
                        .unwrap_or(i32::MAX)
                        .min((y == b).then_some(b).unwrap_or(i32::MAX)),
                    /* skip 2 */
                    (z == a)
                        .then_some(a)
                        .unwrap_or(i32::MAX)
                        .min((z == b).then_some(b).unwrap_or(i32::MAX))
                        .min((z == c).then_some(c).unwrap_or(i32::MAX)),
                ]
            })
            .unwrap()
            .into_iter()
            .min()
            .unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![4, 20, 16, 12, 8], vec![14, 18, 10], -2),
            (vec![3, 5, 5, 3], vec![7, 7], 2),
        ];
        for (nums1, nums2, expected) in tt {
            let output = super::Solution::minimum_added_integer(nums1, nums2);
            assert_eq!(expected, output);
        }
    }
}

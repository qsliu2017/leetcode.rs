#[allow(unused)]
struct Solution;
impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        static mut SET1: [bool; 101] = [false; 101];
        static mut SET2: [bool; 101] = [false; 101];
        unsafe { SET1 = [false; 101] };
        unsafe { SET2 = [false; 101] };
        nums1.iter().for_each(|&n| unsafe {
            SET1[n as usize] = true;
        });
        nums2.iter().for_each(|&n| unsafe {
            SET2[n as usize] = true;
        });
        vec![
            nums1
                .iter()
                .filter(|&&n| unsafe { SET2[n as usize] })
                .count() as i32,
            nums2
                .iter()
                .filter(|&&n| unsafe { SET1[n as usize] })
                .count() as i32,
        ]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![2, 3, 2], vec![1, 2], vec![2, 1]),
            (vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6], vec![3, 4]),
            (vec![3, 4, 2, 3], vec![1, 5], vec![0, 0]),
            (vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6], vec![3, 4]),
        ];
        for (nums1, nums2, expected) in tt {
            let output = super::Solution::find_intersection_values(nums1, nums2);
            assert_eq!(expected, output);
        }
    }
}

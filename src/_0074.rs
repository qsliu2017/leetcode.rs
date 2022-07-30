struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut left, mut right) = (0, m);
        while left < right {
            let mid = (left + right) / 2;
            match target.cmp(&matrix[mid][0]) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Greater => left = mid + 1,
            };
        }
        if left == 0 {
            return false;
        }
        let row = &matrix[left - 1];
        let (mut left, mut right) = (0, n);
        while left < right {
            let mid = (left + right) / 2;
            match target.cmp(&row[mid]) {
                std::cmp::Ordering::Equal => {
                    return true;
                }
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Greater => left = mid + 1,
            };
        }
        if left < n {
            row[left] == target
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::_0074::Solution;

    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3,
                true,
            ),
            (
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13,
                false,
            ),
        ];
        for (matrix, target, output) in tt {
            assert_eq!(Solution::search_matrix(matrix, target), output);
        }
    }
}

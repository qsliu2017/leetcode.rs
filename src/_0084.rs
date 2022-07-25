struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // Self::double_find(heights)
        Self::stack_find(heights)
    }

    fn stack_find(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack = vec![];
        let mut left = vec![0; n];
        for i in 0..n {
            while let Some(top) = stack.last() {
                if heights[*top] >= heights[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            left[i] = stack.last().and_then(|&v| Some(v + 1)).unwrap_or(0);
            stack.push(i);
        }
        stack.clear();
        let mut right = vec![0; n];
        for i in (0..n).rev() {
            while let Some(top) = stack.last() {
                if heights[*top] >= heights[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            right[i] = stack.last().and_then(|&v| Some(v)).unwrap_or(n);
            stack.push(i);
        }

        (0..n)
            .map(|i| (right[i] - left[i]) as i32 * heights[i])
            .max()
            .unwrap_or(0)
    }

    fn double_find(heights: Vec<i32>) -> i32 {
        let mut left_mem: Vec<usize> = (0..heights.len()).collect();
        let mut right_mem: Vec<usize> = (0..heights.len()).map(|i| i + 1).collect();
        (0..heights.len())
            .map(|i| {
                let height = heights[i];
                let mut left = i;
                let mut right = i + 1;
                while left > 0 && heights[left - 1] >= height {
                    right = right_mem[left - 1].max(right);
                    left = left_mem[left - 1].min(left - 1);
                }
                left_mem[i] = left;
                while right < heights.len() && heights[right] >= height {
                    right += 1;
                }
                right_mem[i] = right;
                height * (right - left) as i32
            })
            .max()
            .unwrap()
    }
}
#[cfg(test)]
mod tests {
    use crate::_0084::Solution;

    #[test]
    fn test() {
        let tt = [(vec![2, 1, 5, 6, 2, 3], 10), (vec![2, 4], 4)];
        for (input, output) in tt {
            assert_eq!(Solution::largest_rectangle_area(input), output);
        }
    }
}

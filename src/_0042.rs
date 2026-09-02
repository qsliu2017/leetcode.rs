#[allow(unused)]
struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut watermark = 0;
        let mut acc = 0;
        while left < right {
            if height[left] <= height[right] {
                if height[left] > watermark {
                    acc += (right - left) as i32 * (height[left] - watermark);
                    watermark = height[left];
                }
                acc -= watermark.min(height[left]);
                left += 1;
            } else {
                if height[right] > watermark {
                    acc += (right - left) as i32 * (height[right] - watermark);
                    watermark = height[right];
                }
                acc -= watermark.min(height[right]);
                right -= 1;
            }
        }
        acc
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
            (vec![4, 2, 0, 3, 2, 5], 9),
        ];
        for (input, expected) in tt {
            let output = super::Solution::trap(input);
            assert_eq!(expected, output);
        }
    }
}

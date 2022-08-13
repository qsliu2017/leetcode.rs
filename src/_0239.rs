struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let (heap, counts) = (0..k).into_iter().fold(
            (
                std::collections::BinaryHeap::new(),
                std::collections::HashMap::new(),
            ),
            |(mut heap, mut counts), i| {
                let num = nums[i];
                heap.push(num);
                if let Some(cnt) = counts.get_mut(&num) {
                    *cnt += 1;
                } else {
                    counts.insert(num, 1);
                }
                (heap, counts)
            },
        );
        let window_maxs = vec![*heap.peek().unwrap()];
        (k..nums.len())
            .into_iter()
            .fold(
                (heap, counts, window_maxs),
                |(mut heap, mut counts, mut window_maxs), i| {
                    let (last, first) = (nums[i - k], nums[i]);
                    heap.push(first);
                    *counts.get_mut(&last).unwrap() -= 1;
                    if let Some(cnt) = counts.get_mut(&first) {
                        *cnt += 1;
                    } else {
                        counts.insert(first, 1);
                    }

                    while *counts.get(heap.peek().unwrap()).unwrap() == 0 {
                        heap.pop();
                    }
                    window_maxs.push(*heap.peek().unwrap());

                    (heap, counts, window_maxs)
                },
            )
            .2
    }

    fn slide_max_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut prefix_window = nums;
        let nums = prefix_window.split_off((k - 1) as usize);
        let deque =
            prefix_window
                .into_iter()
                .fold(std::collections::VecDeque::new(), |mut deque, num| {
                    deque.iter_mut().for_each(|n| *n = num.max(*n));
                    deque.push_front(num);
                    deque
                });
        nums.into_iter()
            .fold(
                (deque, Vec::new()),
                |(mut deque, mut max_sliding_window), num| {
                    deque.iter_mut().for_each(|n| *n = num.max(*n));
                    deque.push_front(num);
                    max_sliding_window.push(deque.pop_back().unwrap());
                    (deque, max_sliding_window)
                },
            )
            .1
    }
}
#[cfg(test)]
mod tests {
    use crate::_0239::Solution;

    #[test]
    fn test() {
        let tt = [
            (vec![1, 3, -1, -3, 5, 3, 6, 7], 3, vec![3, 3, 5, 5, 6, 7]),
            (vec![1], 1, vec![1]),
        ];
        for (nums, k, output) in tt {
            assert_eq!(Solution::max_sliding_window(nums, k), output);
        }
    }
}

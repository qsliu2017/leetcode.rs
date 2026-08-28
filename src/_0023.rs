#[allow(unused)]
struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from_vec(value: Vec<i32>) -> Option<Box<Self>> {
        if value.is_empty() {
            return None;
        }
        let mut head = Box::new(Self::new(value[0]));
        let mut tail = &mut head;
        for &val in &value[1..] {
            let node = Box::new(Self::new(val));
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
        return Some(head);
    }
}

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut merged_list = ListNode::new(0);
        let mut heap = lists
            .iter()
            .enumerate()
            .filter_map(|(i, list)| list.as_ref().map(|list| (Reverse(list.val), i)))
            .collect::<BinaryHeap<_>>();
        let mut tail = &mut merged_list;

        while let Some((_, i)) = heap.pop() {
            let Some(mut head) = lists[i].take() else {
                unreachable!()
            };
            let rest = head.next.take();
            lists[i] = rest.inspect(|r| heap.push((Reverse(r.val), i)));

            tail.next = Some(head);
            tail = tail.next.as_mut().unwrap();
        }

        merged_list.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (
                vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]],
                vec![1, 1, 2, 3, 4, 4, 5, 6],
            ),
            (vec![], vec![]),
            (vec![vec![]], vec![]),
        ];
        for (input, expected) in tt {
            let lists = input.into_iter().map(super::ListNode::from_vec).collect();
            let output = super::Solution::merge_k_lists(lists);
            let expected = super::ListNode::from_vec(expected);
            assert_eq!(expected, output);
        }
    }
}

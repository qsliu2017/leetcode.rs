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
    fn from_vec(value: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        for val in value {
            let node = Box::new(ListNode::new(val));
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
        head.next
    }
}
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut split = &mut head;
        for _ in 0..k {
            let Some(current) = split else {
                return head;
            };
            split = &mut current.next;
        }
        let rest = split.take();
        let mut reversed = Self::reverse_k_group(rest, k);

        while let Some(mut first) = head {
            head = first.next.take();
            first.next = reversed;
            reversed = Some(first);
        }
        reversed
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let tt = [
            (vec![1, 2, 3, 4, 5], 2, vec![2, 1, 4, 3, 5]),
            (vec![1, 2, 3, 4, 5], 3, vec![3, 2, 1, 4, 5]),
        ];
        for (head, k, expected) in tt {
            let head = super::ListNode::from_vec(head);
            let output = super::Solution::reverse_k_group(head, k);
            let expected = super::ListNode::from_vec(expected);
            assert_eq!(expected, output);
        }
    }
}

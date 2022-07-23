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
}
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left = vec![];
        let mut right = vec![];

        let mut current = &head;
        while let Some(node) = current {
            current = &node.next;
            if node.val < x {
                left.push(node.val);
            } else {
                right.push(node.val);
            }
        }

        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy.next;
        left.append(&mut right);
        for val in left {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
        }
        dummy.next
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}

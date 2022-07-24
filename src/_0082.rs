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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cnt = vec![0; 201];
        let mut current = &head;
        while let Some(node) = current {
            cnt[(node.val + 100) as usize] += 1;
            current = &node.next;
        }

        let mut dummy = ListNode::new(0);
        let mut next_slot = &mut dummy.next;

        let mut current = &head;
        while let Some(node) = current {
            if cnt[(node.val + 100) as usize] == 1 {
                *next_slot = Some(Box::new(ListNode::new(node.val)));
                next_slot = &mut next_slot.as_mut().unwrap().next;
            }
            current = &node.next;
        }

        dummy.next
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

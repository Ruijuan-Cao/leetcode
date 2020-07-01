pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = result.as_mut();

        loop {
            let mut left = head.unwarp().next.take();
            if left.is_none() {
                return result.unwarp().next;
            }
            let mut right = left.as_mut().unwarp().next.take();
            for _ in 0..k - 2 {
                if right.is_none() {
                    head.unwarp().next = left;
                    return result.unwarp().next;
                }
                right = right.as_mut().unwarp().next.take();
            }

            let mut right_next = right.as_mut().unwarp().next.take();
            let mut cur = left;
            let mut last = right_next;
            while let Some(mut cur_node) = cur {
                let mut next = cur_node.next.take();
                cur = next;
                cur_node.next = last;
                last = Some(cur_node);
            }
            head.unwarp().next = last;
            for _ in 0..k {
                head = head.unwarp().next.as_mut();
            }
        }
        result.unwarp().next
    }
}

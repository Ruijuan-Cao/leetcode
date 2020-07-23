/**
 * [61] Rotate List
 *
 * Given a linked list, rotate the list to the right by k places, where k is non-negative.
 *
 * Example 1:
 *
 *
 * Input: 1->2->3->4->5->NULL, k = 2
 * Output: 4->5->1->2->3->NULL
 * Explanation:
 * rotate 1 steps to the right: 5->1->2->3->4->NULL
 * rotate 2 steps to the right: 4->5->1->2->3->NULL
 *
 *
 * Example 2:
 *
 *
 * Input: 0->1->2->NULL, k = 4
 * Output: 2->0->1->NULL
 * Explanation:
 * rotate 1 steps to the right: 2->0->1->NULL
 * rotate 2 steps to the right: 1->2->0->NULL
 * rotate 3 steps to the right: 0->1->2->NULL
 * rotate 4 steps to the right: 2->0->1->NULL
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/rotate-list/
// discuss: https://leetcode.com/problems/rotate-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k < 1 {
            return head;
        }

        //get len
        let mut len = 0;
        let mut ptr = head.as_ref();
        while let Some(node) = ptr {
            len += 1;
            ptr = node.next.as_ref();
        }

        let loc = len - k % len;
        if loc == len {
            return head;
        }

        let mut head = head;
        let mut ptr = head.as_mut().unwrap();
        for _ in 1..loc {
            ptr = ptr.next.as_mut().unwrap();
        }

        let mut new_head = ptr.next.take();
        let mut ptr = new_head.as_mut();
        while let Some(node) = ptr {
            if node.next.is_none() {
                ptr = Some(node);
                break;
            }
            ptr = node.next.as_mut();
        }
        ptr.unwrap().next = head;
        new_head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_61() {
        assert_eq!(
            Solution::rotate_right(linked![1, 2, 3, 4, 5], 2),
            linked![4, 5, 1, 2, 3]
        );

        assert_eq!(
            Solution::rotate_right(linked![0, 1, 2], 4),
            linked![2, 0, 1]
        );
    }
}

/**
 * [25] Reverse Nodes in k-Group
 *
 * Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.
 *
 * k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes in the end should remain as it is.
 *
 *
 *
 *
 * Example:
 *
 * Given this linked list: 1->2->3->4->5
 *
 * For k = 2, you should return: 2->1->4->3->5
 *
 * For k = 3, you should return: 3->2->1->4->5
 *
 * Note:
 *
 *
 * 	Only constant extra memory is allowed.
 * 	You may not alter the values in the list's nodes, only nodes itself may be changed.
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/reverse-nodes-in-k-group/
// discuss: https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = result.as_mut();

        loop {
            let mut left = head.as_mut().unwrap().next.take();
            if left.is_none() {
                return result.unwrap().next;
            }
            let mut right = left.as_mut();
            for _ in 0..k - 1 {
                right = right.unwrap().next.as_mut();
                if right.is_none() {
                    head.as_mut().unwrap().next = left;
                    return result.unwrap().next;
                }
            }

            let mut right_next = right.as_mut().unwrap().next.take();
            let mut cur = left;
            let mut last = right_next;
            while let Some(mut cur_node) = cur {
                let mut next = cur_node.next.take();
                cur = next;
                cur_node.next = last;
                last = Some(cur_node);
            }
            head.as_mut().unwrap().next = last;
            for _ in 0..k {
                head = head.unwrap().next.as_mut();
            }
        }
        result.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3),
            to_list(vec![3, 2, 1, 4, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 5),
            to_list(vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1]), 1),
            to_list(vec![1])
        );
    }
}

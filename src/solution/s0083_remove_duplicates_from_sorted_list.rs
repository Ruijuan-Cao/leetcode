/**
 * [83] Remove Duplicates from Sorted List
 *
 * Given a sorted linked list, delete all duplicates such that each element appear only once.
 *
 * Example 1:
 *
 *
 * Input: 1->1->2
 * Output: 1->2
 *
 *
 * Example 2:
 *
 *
 * Input: 1->1->2->3->3
 * Output: 1->2->3
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut counter = 1;
        let mut last = head.as_ref().unwrap().val;
        let mut dump = head;
        let mut node = &mut dump.as_mut().unwrap().next;
        loop {
            match node {
                Some(n) if n.val == last => {
                    *node = n.next.take();
                    counter += 1;
                }
                Some(n) => {
                    counter = 0;
                    last = n.val;
                    node = &mut n.next;
                }
                None => return dump,
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_83() {
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 2])),
            to_list(vec![1, 2])
        );
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 2, 3, 3])),
            to_list(vec![1, 2, 3])
        );
    }
}

/**
 * [92] Reverse Linked List II
 *
 * Reverse a linked list from position m to n. Do it in one-pass.
 *
 * Note: 1 &le; m &le; n &le; length of list.
 *
 * Example:
 *
 *
 * Input: 1->2->3->4->5->NULL, m = 2, n = 4
 * Output: 1->4->3->2->5->NULL
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/reverse-linked-list-ii/
// discuss: https://leetcode.com/problems/reverse-linked-list-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() || m >= n || m < 0 || n < 0 {
            return head;
        }
        let mut result = ListNode { next: head, val: 0 };
        let mut pre = Some(Box::new(result));
        let mut cur = pre.as_mut().unwrap();

        let mut count = 0;
        let mut stack = vec![];
        loop {
            if m - 1 == count {
                for _ in m..=n {
                    let mut node = cur.next.take();
                    if node.is_some() {
                        let mut cur_node = node.as_mut().unwrap();
                        let mut rest = cur_node.next.take();

                        cur.next = rest;
                        stack.push(node);
                    }
                }

                let mut nodes = None;
                let mut nodes_tail = None;
                let mut ptr = 0 as *mut Box<ListNode>;

                while !stack.is_empty() {
                    let node = stack.pop().unwrap();
                    if nodes_tail.is_none() {
                        nodes = node;
                        nodes_tail = nodes.as_mut();
                    } else {
                        let tail = nodes_tail.unwrap();
                        tail.next = node;
                        nodes_tail = tail.next.as_mut();
                    }

                    let tmp = nodes_tail.unwrap();
                    ptr = tmp as *mut Box<ListNode>;
                    nodes_tail = Some(tmp);
                }

                let rest = cur.next.take();

                cur.next = nodes;
                unsafe {
                    let nodes_tail = &mut *ptr;
                    nodes_tail.next = rest;
                }
                break;
            }

            if cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
                count += 1;
            } else {
                break;
            }
        }
        pre.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_92() {
        assert_eq!(
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 2, 4),
            to_list(vec![1, 4, 3, 2, 5])
        );
    }
}

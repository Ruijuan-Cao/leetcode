/**
 * [23] Merge k Sorted Lists
 *
 * Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.
 *
 * Example:
 *
 *
 * Input:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * Output: 1->1->2->3->4->4->5->6
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/merge-k-sorted-lists/
// discuss: https://leetcode.com/problems/merge-k-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Node(i32, usize);
impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}
impl std::cmp::Eq for Node {
    // add code here
}
impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for (index, node) in lists.iter().enumerate() {
            node.as_ref().map(|n| heap.push(Node(n.val, index)));
        }
        Solution::next(lists, &mut heap)
    }

    fn next(
        mut lists: Vec<Option<Box<ListNode>>>,
        heap: &mut BinaryHeap<Node>,
    ) -> Option<Box<ListNode>> {
        heap.pop().map(|node| {
            let next = lists[node.1].take().unwrap().next;
            next.as_ref().map(|n| heap.push(Node(n.val, node.1)));
            lists[node.1] = next;
            Box::new(ListNode {
                val: node.0,
                next: Solution::next(lists, heap),
            })
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}

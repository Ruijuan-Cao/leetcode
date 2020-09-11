/**
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * Given a binary tree, return the zigzag level order traversal of its nodes' values. (ie, from left to right, then right to left for the next level and alternate between).
 *
 *
 * For example:<br />
 * Given binary tree [3,9,20,null,null,15,7],<br />
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 *
 *
 * return its zigzag level order traversal as:<br />
 *
 * [
 *   [3],
 *   [20,9],
 *   [15,7]
 * ]
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut cur_level = 0;
        let mut queue = VecDeque::new();
        queue.push_back((cur_level, root.clone()));
        let mut vec = Vec::new();
        while !queue.is_empty() {
            if let Some((level, Some(node))) = queue.pop_front() {
                queue.push_back((level + 1, node.borrow().left.clone()));
                queue.push_back((level + 1, node.borrow().right.clone()));
                if level > cur_level {
                    if cur_level % 2 == 1 {
                        vec.reverse();
                    }
                    result.push(vec);
                    vec = Vec::new();
                    cur_level = level;
                }
                vec.push(node.borrow().val);
            }
        }
        if !vec.is_empty() {
            if cur_level % 2 == 1 {
                vec.reverse();
            }
            result.push(vec);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_103() {
        assert_eq!(
            Solution::zigzag_level_order(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}

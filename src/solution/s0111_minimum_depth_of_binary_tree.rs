/**
 * [111] Minimum Depth of Binary Tree
 *
 * Given a binary tree, find its minimum depth.
 *
 * The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given binary tree [3,9,20,null,null,15,7],
 *
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 * return its minimum depth = 2.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/minimum-depth-of-binary-tree/
// discuss: https://leetcode.com/problems/minimum-depth-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut queue = VecDeque::new();
        queue.push_back((1, root.clone()));
        while !queue.is_empty() {
            if let Some((level, Some(node))) = queue.pop_front() {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return level;
                }
                queue.push_back((level + 1, node.borrow().left.clone()));
                queue.push_back((level + 1, node.borrow().right.clone()));
            }
        }
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111() {
        assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
    }
}

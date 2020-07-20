/**
 * [45] Jump Game II
 *
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 *
 * Each element in the array represents your maximum jump length at that position.
 *
 * Your goal is to reach the last index in the minimum number of jumps.
 *
 * Example:
 *
 *
 * Input: [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2.
 *     Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 * Note:
 *
 * You can assume that you can always reach the last index.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-ii/
// discuss: https://leetcode.com/problems/jump-game-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO: shortest path from backward
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut pre, mut cur) = (-1i32, 0i32);
        let mut step = 0;
        for i in 0..nums.len() as i32 {
            if cur >= nums.len() as i32 - 1 {
                return step;
            } else if pre < i as i32 && i <= cur {
                step += 1;
                pre = cur;
            }
            if i + nums[i as usize] > cur {
                cur = i + nums[i as usize];
            }
        }
        step
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}

/**
 * [81] Search in Rotated Sorted Array II
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).
 *
 * You are given a target value to search. If found in the array return true, otherwise return false.
 *
 * Example 1:
 *
 *
 * Input: nums = [2,5,6,0,0,1,2], target = 0
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,5,6,0,0,1,2], target = 3
 * Output: false
 *
 * Follow up:
 *
 *
 * 	This is a follow up problem to <a href="/problems/search-in-rotated-sorted-array/description/">Search in Rotated Sorted Array</a>, where nums may contain duplicates.
 * 	Would this affect the run-time complexity? How and why?
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
// discuss: https://leetcode.com/problems/search-in-rotated-sorted-array-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        for i in 0..nums.len() {
            if nums[i] == target {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_81() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }
}

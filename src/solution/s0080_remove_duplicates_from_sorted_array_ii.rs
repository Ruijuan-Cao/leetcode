/**
 * [80] Remove Duplicates from Sorted Array II
 *
 * Given a sorted array nums, remove the duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> such that duplicates appeared at most twice and return the new length.
 *
 * Do not allocate extra space for another array, you must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 *
 * Example 1:
 *
 *
 * Given nums = [1,1,1,2,2,3],
 *
 * Your function should return length = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
 *
 * It doesn't matter what you leave beyond the returned length.
 *
 * Example 2:
 *
 *
 * Given nums = [0,0,1,1,1,1,2,3,3],
 *
 * Your function should return length = 7, with the first seven elements of nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively.
 *
 * It doesn't matter what values are set beyond the returned length.
 *
 *
 * Clarification:
 *
 * Confused why the returned value is an integer but your answer is an array?
 *
 * Note that the input array is passed in by reference, which means modification to the input array will be known to the caller as well.
 *
 * Internally you can think of this:
 *
 *
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeDuplicates(nums);
 *
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len elements.
 * for (int i = 0; i < len; i++) {
 *     print(nums[i]);
 * }
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut last = nums[0];
        let mut counter = 0;
        let mut i = 1;
        while i < nums.len() {
            if last == nums[i] {
                if counter == 0 {
                    counter = 1;
                    i += 1;
                } else {
                    nums.remove(i);
                }
            } else {
                last = nums[i];
                counter = 0;
                i += 1;
            }
        }
        nums.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_80() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
    }
}

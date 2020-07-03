/**
 * [31] Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 *
 * If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
 *
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> and use only constant extra memory.
 *
 * Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
 *
 * 1,2,3 &rarr; 1,3,2<br />
 * 3,2,1 &rarr; 1,2,3<br />
 * 1,1,5 &rarr; 1,5,1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-permutation/
// discuss: https://leetcode.com/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        //find the reverse index,max value
        let len = nums.len();
        let mut i = len - 1;
        let mut pre = -1;
        while i >= 0 {
            if nums[i] < pre {
                break;
            } else {
                pre = nums[i];
                if i > 0 {
                    i -= 1;
                } else {
                    return nums.reverse();
                }
            }
        }
        //find value bigger than nums[i], swap and reverse
        let mut j = len - 1;
        while j > i {
            if nums[j] > nums[i] {
                nums.swap(i, j);
                break;
            }
            j -= 1;
        }
        nums[(i + 1)..].reverse();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut vec1 = vec![1, 2, 3, 4, 5];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3, 5, 4]);

        let mut vec2 = vec![5, 4, 3, 2, 1];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);

        let mut vec2 = vec![1, 3, 2];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![2, 1, 3]);
    }

}

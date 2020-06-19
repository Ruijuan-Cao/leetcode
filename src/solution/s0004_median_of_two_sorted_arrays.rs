/**
 * [4] Median of Two Sorted Arrays
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 *
 * Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
 *
 * You may assume nums1 and nums2 cannot be both empty.
 *
 * Example 1:
 *
 *
 * nums1 = [1, 3]
 * nums2 = [2]
 *
 * The median is 2.0
 *
 *
 * Example 2:
 *
 *
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 *
 * The median is (2 + 3)/2 = 2.5
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO: nth slice
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut odd = true;
        let mut num = nums1.len() + nums2.len();
        if num % 2 == 0 {
            odd = false;
        }
        let mut pre = 0;
        let mut cur = 0;
        num = (num / 2) as usize;
        let (mut mid, mut x, mut y) = (0, 0, 0);
        let mut geted = false;
        while x < nums1.len() && y < nums2.len() {
            if nums1[x] > nums2[y] {
                pre = cur;
                cur = nums2[y];
                y += 1;
            } else {
                pre = cur;
                cur = nums1[x];
                x += 1;
            }
            if mid < num {
                mid += 1;
            } else {
                geted = true;
                break;
            }
        }
        if x == nums1.len() {
            while y < nums2.len() {
                pre = cur;
                pre = nums2[y];
                y += 1;

                if mid < num {
                    mid += 1;
                } else {
                    geted = true;
                    break;
                }
            }
        } else {
            while x < nums1.len() {
                pre = cur;
                pre = nums1[x];
                x += 1;

                if mid < num {
                    mid += 1;
                } else {
                    geted = true;
                    break;
                }
            }
        }
        if odd {
            return cur as f64;
        } else {
            return (pre as f64 + cur as f64) / 2.0;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: implementation
    #[test]
    fn test_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}

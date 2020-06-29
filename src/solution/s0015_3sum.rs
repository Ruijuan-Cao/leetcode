/**
 * [15] 3Sum
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
 *
 * Note:
 *
 * The solution set must not contain duplicate triplets.
 *
 * Example:
 *
 *
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 *
 * A solution set is:
 * [
 *   [-1, 0, 1],
 *   [-1, -1, 2]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn quick_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut l = left;
        let mut r = right;
        let temp = nums[left];
        while l < r {
            //must from right to left
            while l < r && nums[r] >= temp {
                r -= 1;
            }
            while l < r && nums[l] <= temp {
                l += 1;
            }
            //return to iter again
            nums.swap(l, r);
        }
        nums.swap(left, l);
        if l > 1 {
            Solution::quick_sort(nums, left, l - 1);
        }
        Solution::quick_sort(nums, r + 1, right);
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 3 {
            return result;
        }
        let len = nums.len();
        let mut nums_mut = nums;
        //Solution::quick_sort(&mut nums_mut, 0, len - 1);
        nums_mut.sort();

        for i in 0..len - 2 {
            if i > 0 && nums_mut[i] == nums_mut[i - 1] {
                continue;
            }
            let sum = 0 - nums_mut[i];
            let mut x = i + 1;
            let mut y = len - 1;
            while x < y {
                if nums_mut[x] + nums_mut[y] == sum {
                    let re = vec![nums_mut[i], nums_mut[x], nums_mut[y]];
                    result.push(re);
                    //remove the same number
                    while x < y && nums_mut[x] == nums_mut[x + 1] {
                        x += 1;
                    }
                    while x < y && nums_mut[y] == nums_mut[y - 1] {
                        y -= 1;
                    }
                    x += 1;
                    y -= 1;
                } else if nums_mut[x] + nums_mut[y] < sum {
                    x += 1;
                } else {
                    y -= 1;
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![
                -7, -4, -6, 6, 4, -6, -9, -10, -7, 5, 3, -1, -5, 8, -1, -2, -8, -1, 5, -3, -5, 4,
                2, -5, -4, 4, 7
            ]),
            vec![
                vec![-10, 2, 8],
                vec![-10, 3, 7],
                vec![-10, 4, 6],
                vec![-10, 5, 5],
                vec![-9, 2, 7],
                vec![-9, 3, 6],
                vec![-9, 4, 5],
                vec![-8, 2, 6],
                vec![-8, 3, 5],
                vec![-8, 4, 4],
                vec![-7, -1, 8],
                vec![-7, 2, 5],
                vec![-7, 3, 4],
                vec![-6, -2, 8],
                vec![-6, -1, 7],
                vec![-6, 2, 4],
                vec![-5, -3, 8],
                vec![-5, -2, 7],
                vec![-5, -1, 6],
                vec![-5, 2, 3],
                vec![-4, -4, 8],
                vec![-4, -3, 7],
                vec![-4, -2, 6],
                vec![-4, -1, 5],
                vec![-3, -2, 5],
                vec![-3, -1, 4],
                vec![-2, -1, 3],
                vec![-1, -1, 2]
            ]
        );
        assert_eq!(
            Solution::three_sum(vec![2, 0, -2, -5, -5, -3, 2, -4]),
            vec![vec![-4, 2, 2], vec![-2, 0, 2]]
        );
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![[0, 0, 0]]);
        let empty_vec: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(vec![]), empty_vec);
    }
}

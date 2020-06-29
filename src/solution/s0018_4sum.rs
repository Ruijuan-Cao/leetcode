/**
 * [18] 4Sum
 *
 * Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
 *
 * Note:
 *
 * The solution set must not contain duplicate quadruplets.
 *
 * Example:
 *
 *
 * Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
 *
 * A solution set is:
 * [
 *   [-1,  0, 0, 1],
 *   [-2, -1, 1, 2],
 *   [-2,  0, 0, 2]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/4sum/
// discuss: https://leetcode.com/problems/4sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO: change to faster N^3 solution... maybe
// this is a N^2 * logN solution, but slower than N^3 solution
// iterate all combinations and the sum of 2 elements, then use one-round hash
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut nums = nums;
        if nums.len() < 4 {
            return result;
        }
        nums.sort();

        for mut i in 0..nums.len() - 3 {
            if result.len() > 0 && i > 0 && nums[i] == nums[i - 1] {
                i += 1;
                continue;
            }

            for mut j in (i + 1)..nums.len() - 2 {
                if result.len() > 0 && j > 0 && nums[j] == nums[j - 1] {
                    j += 1;
                    continue;
                }

                let mut x = i + 1;
                let mut y = nums.len() - 1;
                let sum = target - nums[i] - nums[j];
                while x < y {
                    if nums[x] + nums[y] == sum {
                        result.push(vec![nums[i], nums[j], nums[x], nums[y]]);
                        while x < y && nums[x] == nums[x + 1] {
                            x += 1;
                        }
                        while x < y && nums[y] == nums[y - 1] {
                            y -= 1;
                        }
                        x += 1;
                        y -= 1;
                    } else if nums[x] + nums[y] > sum {
                        y -= 1;
                    } else {
                        x += 1;
                    }
                }
                while j < nums.len() - 2 && nums[j] == nums[j + 1] {
                    j += 1;
                }
            }
            while i < nums.len() - 3 && nums[i] == nums[i + 1] {
                i += 1;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: build a macro for arbitrary match
    #[test]
    fn test_18() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(Solution::four_sum(vec![0, 0, 0, 0], 0), vec![[0, 0, 0, 0]]);
        assert_eq!(
            Solution::four_sum(vec![-3, -1, 0, 2, 4, 5], 0),
            vec![[-3, -1, 0, 4]]
        );
    }
}

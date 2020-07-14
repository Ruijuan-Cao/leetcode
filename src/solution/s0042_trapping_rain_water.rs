/**
 * [42] Trapping Rain Water
 *
 * Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it is able to trap after raining.
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png" style="width: 412px; height: 161px;" /><br />
 * <small>The above elevation map is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped. Thanks Marcos for contributing this image!</small>
 *
 * Example:
 *
 *
 * Input: [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water/
// discuss: https://leetcode.com/problems/trapping-rain-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut sum = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let cur_min = std::cmp::min(height[left], height[right]);
            if height[left] < height[right] {
                left += 1;
                while left < right && height[left] < cur_min {
                    sum += cur_min - height[left];
                    left += 1;
                }
            } else {
                right -= 1;
                while left < right && height[right] < cur_min {
                    sum += cur_min - height[right];
                    right -= 1;
                }
            }
        }
        sum
    }
    pub fn trap2(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut sum = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut level = 0;
        while left < right {
            let cur_min = std::cmp::min(height[left], height[right]);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
            if cur_min > level {
                level = cur_min;
            }
            sum += level - cur_min;
        }
        sum
    }

    pub fn trap3(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut sum = 0;
        let mut i = 0;
        let mut stack = Vec::new();
        while i < height.len() {
            if stack.len() == 0 || height[i] < height[*stack.last().unwrap()] {
                stack.push(i);
                i += 1;
            } else {
                let top = stack.pop().unwrap();
                if stack.len() == 0 {
                    continue;
                }
                let last = *stack.last().unwrap();
                let cur_min = std::cmp::min(height[last], height[i]);
                sum += (cur_min - height[top]) * (i - last - 1) as i32;
            }
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {
        assert_eq!(Solution::trap2(vec![]), 0);
        assert_eq!(Solution::trap3(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}

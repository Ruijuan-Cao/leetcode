/**
 * [7] Reverse Integer
 *
 * Given a 32-bit signed integer, reverse digits of an integer.
 *
 * Example 1:
 *
 *
 * Input: 123
 * Output: 321
 *
 *
 * Example 2:
 *
 *
 * Input: -123
 * Output: -321
 *
 *
 * Example 3:
 *
 *
 * Input: 120
 * Output: 21
 *
 *
 * Note:<br />
 * Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-integer/
// discuss: https://leetcode.com/problems/reverse-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut result = 0;
        let mut num = x;
        while num != 0 {
            let div = num % 10;
            if (result > std::i32::MAX / 10
                || (result == std::i32::MAX / 10 && div > 7)
                || result < std::i32::MIN / 10
                || (result == std::i32::MIN / 10 && div < -8))
            {
                return 0;
            }

            result = result * 10 + div;
            num = num / 10;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        // assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
        assert_eq!(Solution::reverse(1563847412), 0);
    }
}

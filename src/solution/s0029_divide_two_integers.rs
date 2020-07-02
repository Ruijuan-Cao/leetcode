/**
 * [29] Divide Two Integers
 *
 * Given two integers dividend and divisor, divide two integers without using multiplication, division and mod operator.
 *
 * Return the quotient after dividing dividend by divisor.
 *
 * The integer division should truncate toward zero.
 *
 * Example 1:
 *
 *
 * Input: dividend = 10, divisor = 3
 * Output: 3
 *
 * Example 2:
 *
 *
 * Input: dividend = 7, divisor = -3
 * Output: -2
 *
 * Note:
 *
 *
 * 	Both dividend and divisor will be 32-bit signed integers.
 * 	The divisor will never be 0.
 * 	Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. For the purpose of this problem, assume that your function returns 2^31 - 1 when the division result overflows.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-two-integers/
// discuss: https://leetcode.com/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            return std::i32::MAX;
        }
        let mut is_positive = false;
        if (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0) {
            is_positive = true;
        }

        let mut result = 0;
        let mut dividend = (dividend as i64).abs();
        let mut divisor = (divisor as i64).abs();
        while dividend - divisor >= 0 {
            let mut n = divisor;
            let mut i = 1;
            while dividend >= (n << 1) {
                n <<= 1;
                i <<= 1;
            }
            result += i;
            dividend -= n;
        }
        if !is_positive {
            result = -result;
        }
        if result > std::i32::MAX as i64 {
            return std::i32::MAX;
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {
        assert_eq!(Solution::divide(1, 1), 1);
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(std::i32::MIN, -1), std::i32::MAX);
        assert_eq!(Solution::divide(std::i32::MIN, 1), std::i32::MIN);
    }
}

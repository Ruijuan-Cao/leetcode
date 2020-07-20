/**
 * [50] Pow(x, n)
 *
 * Implement <a href="http://www.cplusplus.com/reference/valarray/pow/" target="_blank">pow(x, n)</a>, which calculates x raised to the power n (x^<span style="font-size:10.8333px">n</span>).
 *
 * Example 1:
 *
 *
 * Input: 2.00000, 10
 * Output: 1024.00000
 *
 *
 * Example 2:
 *
 *
 * Input: 2.10000, 3
 * Output: 9.26100
 *
 *
 * Example 3:
 *
 *
 * Input: 2.00000, -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 *
 *
 * Note:
 *
 *
 * 	-100.0 < x < 100.0
 * 	n is a 32-bit signed integer, within the range [-2^31, 2^31 - 1]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/powx-n/
// discuss: https://leetcode.com/problems/powx-n/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut m = n;
        if n < 0 {
            m = -n - 1;
        }

        let mut p = 1.0;
        let mut q = x;
        while m > 0 {
            if m & 1 != 0 {
                p *= q;
            }
            q *= q;
            m = m / 2;
        }

        if n < 0 {
            return 1.0 / p / x;
        } else {
            return p;
        }
    }
    pub fn my_pow_overtime(x: f64, n: i32) -> f64 {
        if x <= -100.0 || x >= 100.0 {
            return 0.0;
        }
        if n == 0 {
            return 1.0;
        }
        let mut result = 1.0;
        let mut n = n;
        let mut negtive = false;
        if n < 0 {
            n = -n;
            negtive = true;
        }

        for _ in 0..n {
            result *= x;
        }
        if negtive {
            return 1.0 / result;
        } else {
            return result;
        }
        // x.powi(n)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_50() {
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
        assert_eq!(Solution::my_pow(2.0, 4), 16.0);
        assert_eq!(Solution::my_pow(2.0, 5), 32.0);
        assert_eq!(Solution::my_pow(2.0, 1), 2.0);
        assert_eq!(Solution::my_pow(2.0, -1), 0.5);
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    }
}

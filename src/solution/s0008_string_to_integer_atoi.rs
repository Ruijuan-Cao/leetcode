/**
 * [8] String to Integer (atoi)
 *
 * Implement <code><span>atoi</span></code> which converts a string to an integer.
 *
 * The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
 *
 * The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
 *
 * If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
 *
 * If no valid conversion could be performed, a zero value is returned.
 *
 * Note:
 *
 * <ul>
 * 	<li>Only the space character <code>&#39; &#39;</code> is considered as whitespace character.</li>
 * 	<li>Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. If the numerical value is out of the range of representable values, INT_MAX (2^31 - 1) or INT_MIN (-2^31) is returned.</li>
 * </ul>
 *
 * Example 1:
 *
 *
 * Input: "42"
 * Output: 42
 *
 *
 * Example 2:
 *
 *
 * Input: "   -42"
 * Output: -42
 * Explanation: The first non-whitespace character is &#39;-&#39;, which is the minus sign.
 *              Then take as many numerical digits as possible, which gets 42.
 *
 *
 * Example 3:
 *
 *
 * Input: "4193 with words"
 * Output: 4193
 * Explanation: Conversion stops at digit &#39;3&#39; as the next character is not a numerical digit.
 *
 *
 * Example 4:
 *
 *
 * Input: "words and 987"
 * Output: 0
 * Explanation: The first non-whitespace character is &#39;w&#39;, which is not a numerical
 *              digit or a +/- sign. Therefore no valid conversion could be performed.
 *
 * Example 5:
 *
 *
 * Input: "-91283472332"
 * Output: -2147483648
 * Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
 *              Thefore INT_MIN (-2^31) is returned.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-to-integer-atoi/
// discuss: https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut result = 0;
        let chars: Vec<char> = str.chars().collect();
        let mut first_space = true;
        let mut negative = false;
        let mut has_signal = false;
        let mut has_dig = false;
        for i in 0..str.len() {
            if chars[i] >= '0' && chars[i] <= '9' {
                has_dig = true;
                let val = chars[i] as i32 - '0' as i32;

                if result > std::i32::MAX / 10 || (result == std::i32::MAX / 10 && val > 7) {
                    if negative {
                        return std::i32::MIN;
                    } else {
                        return std::i32::MAX;
                    }
                }
                result = result * 10 + val;
            } else {
                if has_dig || has_signal {
                    break;
                }
                if chars[i] == '+' {
                    has_signal = true;
                    continue;
                } else if chars[i] == '-' {
                    has_signal = true;
                    negative = true;
                    continue;
                } else if chars[i] == ' ' {
                    if first_space {
                        continue;
                    } else {
                        break;
                    }
                } else {
                    if first_space {
                        first_space = false;
                    }
                    break;
                }
            }
        }
        if negative {
            0 - result
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("aa".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
        assert_eq!(Solution::my_atoi("3.14".to_string()), 3);
        assert_eq!(Solution::my_atoi(".1".to_string()), 0);
        assert_eq!(Solution::my_atoi("+1".to_string()), 1);
        assert_eq!(Solution::my_atoi("+-1".to_string()), 0);
        assert_eq!(Solution::my_atoi("   +0 123".to_string()), 0);
        assert_eq!(Solution::my_atoi("-2147483649".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("0-2".to_string()), 0);
    }
}

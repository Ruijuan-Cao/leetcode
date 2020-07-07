/**
 * [32] Longest Valid Parentheses
 *
 * Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.
 *
 * Example 1:
 *
 *
 * Input: "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()"
 *
 *
 * Example 2:
 *
 *
 * Input: ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()"
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-valid-parentheses/
// discuss: https://leetcode.com/problems/longest-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// time: O(N) space: O(1)
impl Solution {
    pub fn longest_valid_parentheses2(s: String) -> i32 {
        let len = s.len();
        if len < 2 {
            return 0;
        }
        let mut dp = vec![0; len];
        let mut max = 0;
        for i in 1..len {
            if s.chars().nth(i) == Some(')') {
                if s.chars().nth(i - 1) == Some('(') {
                    if i > 2 {
                        dp[i] = dp[i - 2] + 2;
                    } else {
                        dp[i] = 2;
                    }
                } else {
                    let pre_seq_start = i - dp[i - 1];
                    if pre_seq_start > 0 && s.chars().nth(pre_seq_start - 1) == Some('(') {
                        if pre_seq_start > 1 {
                            dp[i] = dp[i - 1] + dp[pre_seq_start - 2] + 2;
                        } else {
                            dp[i] = dp[i - 1] + 2;
                        }
                    }
                }
                if dp[i] > max {
                    max = dp[i];
                }
            }
        }
        max as i32
    }

    pub fn longest_valid_parentheses_stack(s: String) -> i32 {
        let len = s.len();
        if len < 2 {
            return 0;
        }
        let mut stack: Vec<i32> = Vec::new();
        stack.push(-1);
        let mut result = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..len {
            if chars[i] == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                match stack.last() {
                    Some(index) => {
                        result = result.max(i as i32 - *index);
                    }
                    None => {
                        stack.push(i as i32);
                    }
                }
            }
        }
        result
    }

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let len = s.len();
        if len < 2 {
            return 0;
        }
        let (mut left, mut right) = (0, 0);
        let mut max = 0;
        for i in 0..len {
            if s.chars().nth(i) == Some('(') {
                left += 1;
            } else {
                right += 1;
            }
            if left == right {
                if left + right > max {
                    max = left + right;
                }
            } else if right >= left {
                left = 0;
                right = 0;
            }
        }
        left = 0;
        right = 0;
        let mut i = len - 1;
        while i >= 0 {
            if s.chars().nth(i) == Some('(') {
                left += 1;
            } else {
                right += 1;
            }
            if left == right {
                if left + right > max {
                    max = left + right;
                }
            } else if left >= right {
                left = 0;
                right = 0;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        max as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses(")(".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses("(((((()()".to_string()),
            4
        );
        assert_eq!(
            Solution::longest_valid_parentheses("((((((((()))".to_string()),
            6
        );
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses(")()(((())))(".to_string()),
            10
        );
        assert_eq!(
            Solution::longest_valid_parentheses(")(())))(())())".to_string()),
            6
        );
        assert_eq!(
            Solution::longest_valid_parentheses("(()))())(".to_string()),
            4
        );
        assert_eq!(Solution::longest_valid_parentheses(")(()(()(((())(((((()()))((((()()(()()())())())()))()()()())(())()()(((()))))()((()))(((())()((()()())((())))(())))())((()())()()((()((())))))((()(((((()((()))(()()(())))((()))()))())".to_string()), 132);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("()(())".to_string()), 6);
        assert_eq!(
            Solution::longest_valid_parentheses("()((())()".to_string()),
            6
        );
        assert_eq!(
            Solution::longest_valid_parentheses("()))(()(()))(".to_string()),
            8
        );
    }
}

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
        if s.len() < 2 {
            return 0;
        }
        let mut stack = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        let mut num = 0;
        let mut pair_index = Vec::new();
        for i in 0..s.len() {
            if chars[i] == '(' {
                stack.push('(');
            } else {
                match stack.last() {
                    Some(ch) => {
                        if *ch == '(' {
                            stack.pop();
                            num += 2;
                            pair_index.push(i);
                        } else {
                            stack.push(')');
                        }
                    }
                    None => {
                        stack.push(')');
                    }
                }
            }
        }
        let size = pair_index.len();
        if size < 1 {
            return 0;
        }

        let mut continus_num = 0;
        for i in 1..size {
            if pair_index[i] - pair_index[i - 1] == 1 {
                continus_num += 1;
            }
        }
        // println!("{:?}:{:?}-{:?}", s, pair_index, continus_num);
        let mut max_num = 0;
        let mut cur_group_size = 1;
        let mut cur_continus_num = continus_num;
        let mut i = 1;
        for i in 1..size {
            let diff = pair_index[i] - pair_index[i - 1];
            if diff == 1 {
                continus_num -= 1;
            }
            // println!("{}-{}-{}", diff, continus_num, cur_continus_num);
            if (continus_num > 0 && diff > 2 + continus_num) || (continus_num == 0 && diff > 2) {
                max_num = max_num.max(cur_group_size);
                cur_group_size = 1;
                continus_num = cur_continus_num;
            } else {
                cur_group_size += 1;
            }
            // println!("--------{}-{}-{}", i, cur_group_size, max_num);
        }

        max_num = max_num.max(cur_group_size);

        max_num as i32 * 2
    }

    pub fn longest_valid_parentheses(s: String) -> i32 {
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

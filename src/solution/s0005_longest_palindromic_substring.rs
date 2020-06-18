/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
 *
 * Example 1:
 *
 *
 * Input: "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: "cbbd"
 * Output: "bb"
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {}
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
        assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_owned()), "");
    }
}

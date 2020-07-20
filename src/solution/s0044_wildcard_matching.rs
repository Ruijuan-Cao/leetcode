/**
 * [44] Wildcard Matching
 *
 * Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*'.
 *
 *
 * '?' Matches any single character.
 * '*' Matches any sequence of characters (including the empty sequence).
 *
 *
 * The matching should cover the entire input string (not partial).
 *
 * Note:
 *
 *
 * 	s could be empty and contains only lowercase letters a-z.
 * 	p could be empty and contains only lowercase letters a-z, and characters like <font face="monospace">?</font> or *.
 *
 *
 * Example 1:
 *
 *
 * Input:
 * s = "aa"
 * p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 *
 * Example 2:
 *
 *
 * Input:
 * s = "aa"
 * p = "*"
 * Output: true
 * Explanation: '*' matches any sequence.
 *
 *
 * Example 3:
 *
 *
 * Input:
 * s = "cb"
 * p = "?a"
 * Output: false
 * Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
 *
 *
 * Example 4:
 *
 *
 * Input:
 * s = "adceb"
 * p = "*a*b"
 * Output: true
 * Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
 *
 *
 * Example 5:
 *
 *
 * Input:
 * s = "acdcb"
 * p = "a*c?b"
 * Output: false
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/wildcard-matching/
// discuss: https://leetcode.com/problems/wildcard-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        //result[i][j]-if s[0..i] and p[0..j] match
        let mut result = vec![vec![false; p.len() + 1]; s.len() + 1];
        result[0][0] = true;
        for i in 0..s.len() + 1 {
            for j in 1..p.len() + 1 {
                if p[j - 1] == '*' {
                    result[i][j] = result[i][j - 1] || (i > 0 && result[i - 1][j]);
                } else {
                    result[i][j] =
                        i > 0 && result[i - 1][j - 1] && (s[i - 1] == p[j - 1] || p[j - 1] == '?');
                }
            }
        }
        result[s.len()][p.len()]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_44() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
        assert_eq!(
            Solution::is_match("cb".to_string(), "?a".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("adceb".to_string(), "*a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("acdcb".to_string(), "a*c?b".to_string()),
            false
        );
    }
}

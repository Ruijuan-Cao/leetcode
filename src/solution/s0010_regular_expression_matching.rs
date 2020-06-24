/**
 * [10] Regular Expression Matching
 *
 * Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.
 *
 *
 * '.' Matches any single character.
 * '*' Matches zero or more of the preceding element.
 *
 *
 * The matching should cover the entire input string (not partial).
 *
 * Note:
 *
 *
 * 	s could be empty and contains only lowercase letters a-z.
 * 	p could be empty and contains only lowercase letters a-z, and characters like . or *.
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
 * p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the precedeng element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 *
 *
 * Example 3:
 *
 *
 * Input:
 * s = "ab"
 * p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 *
 * Example 4:
 *
 *
 * Input:
 * s = "aab"
 * p = "c*a*b"
 * Output: true
 * Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore it matches "aab".
 *
 *
 * Example 5:
 *
 *
 * Input:
 * s = "mississippi"
 * p = "mis*is*p*."
 * Output: false
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/regular-expression-matching/
// discuss: https://leetcode.com/problems/regular-expression-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO: NFA
impl Solution {
    // has error, can't handle all conditions,"ab*a*c*a"-"aaa"should be true, but false
    pub fn is_match2(s: String, p: String) -> bool {
        if p == "*" {
            return false;
        }
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        if s.len() == 0 {
            if p.len() == 0 {
                return true;
            } else if p.len() == 2 && p[0] != '*' && p[1] == '*' {
                return true;
            } else {
                return false;
            }
        }
        let mut i = 0;
        let mut j = 0;
        let mut pre_ch = '\n';
        let mut has_char = false;
        while i < s.len() && j < p.len() {
            if s[i] == p[j] || p[j] == '.' {
                println!("0-{}-{}-{}-{}", i, j, s[i], p[j]);

                has_char = true;
                pre_ch = s[i];
                if p[j] == '.' {
                    pre_ch = '.';
                }
                i += 1;
                j += 1;
            } else if p[j] == '*' {
                println!("1-{}-{}-{}-{}", i, j, s[i], p[j]);

                if has_char == false {
                    return false;
                }
                if s[i] == pre_ch || pre_ch == '.' {
                    i += 1;
                    if i == s.len() || (j + 1 < p.len() && s[i] == p[j + 1]) {
                        j += 1;
                    }
                } else {
                    j += 1;
                    has_char = false;
                }
            } else {
                println!("2-{}-{}-{}-{}", i, j, s[i], p[j]);

                if j + 1 < p.len() && p[j + 1] == '*' {
                    j += 2;
                    has_char = false;
                } else {
                    return false;
                }
            }
        }
        if i == s.len() && j == p.len() {
            return true;
        } else {
            return false;
        }
    }

    //by dp
    pub fn is_match(s: String, p: String) -> bool {
        if p == "*" {
            return false;
        }
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        //result[i][j]-if s[0..i] and p[0..j] match
        let mut result = vec![vec![false; p.len() + 1]; s.len() + 1];
        result[0][0] = true;
        for i in 0..s.len() + 1 {
            for j in 1..p.len() + 1 {
                if j > 1 && p[j - 1] == '*' {
                    result[i][j] = result[i][j - 2]
                        || (i > 0 && (s[i - 1] == p[j - 2] || p[j - 2] == '.') && result[i - 1][j]);
                } else {
                    result[i][j] =
                        i > 0 && result[i - 1][j - 1] && (s[i - 1] == p[j - 1] || p[j - 1] == '.');
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
    fn test_10() {
        assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
        assert_eq!(Solution::is_match("".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("".to_string(), "*".to_string()), false);
        assert_eq!(Solution::is_match("".to_string(), ".".to_string()), false);
        assert_eq!(Solution::is_match("".to_string(), "a*a*".to_string()), true);
        assert_eq!(Solution::is_match("".to_string(), "aa*".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("aaa".to_string(), "a*a".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("ab".to_string(), ".*c".to_string()),
            false
        );

        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*".to_string()),
            false
        );
    }
}

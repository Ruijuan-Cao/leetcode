/**
 * [87] Scramble String
 *
 * Given a string s1, we may represent it as a binary tree by partitioning it to two non-empty substrings recursively.
 *
 * Below is one possible representation of s1 = "great":
 *
 *
 *     great
 *    /    \
 *   gr    eat
 *  / \    /  \
 * g   r  e   at
 *            / \
 *           a   t
 *
 *
 * To scramble the string, we may choose any non-leaf node and swap its two children.
 *
 * For example, if we choose the node "gr" and swap its two children, it produces a scrambled string "rgeat".
 *
 *
 *     rgeat
 *    /    \
 *   rg    eat
 *  / \    /  \
 * r   g  e   at
 *            / \
 *           a   t
 *
 *
 * We say that "rgeat" is a scrambled string of "great".
 *
 * Similarly, if we continue to swap the children of nodes "eat" and "at", it produces a scrambled string "rgtae".
 *
 *
 *     rgtae
 *    /    \
 *   rg    tae
 *  / \    /  \
 * r   g  ta  e
 *        / \
 *       t   a
 *
 *
 * We say that "rgtae" is a scrambled string of "great".
 *
 * Given two strings s1 and s2 of the same length, determine if s2 is a scrambled string of s1.
 *
 * Example 1:
 *
 *
 * Input: s1 = "great", s2 = "rgeat"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s1 = "abcde", s2 = "caebd"
 * Output: false
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/scramble-string/
// discuss: https://leetcode.com/problems/scramble-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }

        if s1 == s2 {
            return true;
        }

        let mut char1: Vec<char> = s1.chars().collect();
        let mut char2: Vec<char> = s2.chars().collect();

        char1.sort();
        char2.sort();
        for i in 0..s1.len() {
            if char1[i] != char2[i] {
                return false;
            }
        }
        for i in 1..s1.len() {
            if Self::is_scramble(s1[0..i].to_string(), s2[0..i].to_string())
                && Self::is_scramble(s1[i..].to_string(), s2[i..].to_string())
            {
                return true;
            }

            if Self::is_scramble(s1[0..i].to_string(), s2[(s1.len() - i)..].to_string())
                && Self::is_scramble(s1[i..].to_string(), s2[0..(s1.len() - i)].to_string())
            {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_87() {
        assert_eq!(
            Solution::is_scramble("great".to_string(), "rgeat".to_string()),
            true
        );

        assert_eq!(
            Solution::is_scramble("abcde".to_string(), "caebd".to_string()),
            false
        );
    }
}

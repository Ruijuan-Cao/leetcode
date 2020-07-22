/**
 * [58] Length of Last Word
 *
 * Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word in the string.
 *
 * If the last word does not exist, return 0.
 *
 * Note: A word is defined as a character sequence consists of non-space characters only.
 *
 * Example:
 *
 * Input: "Hello World"
 * Output: 5
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/length-of-last-word/
// discuss: https://leetcode.com/problems/length-of-last-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut s = s.trim();
        if s.len() < 1 {
            return 0;
        }
        let chars: Vec<char> = s.trim().chars().collect();
        let mut i = chars.len() as i32 - 1;
        while i >= 0 && chars[i as usize] != ' ' {
            i -= 1;
        }
        chars.len() as i32 - 1 - i
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_58() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word("       ".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word("".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word("     rrrrr  ".to_owned()), 5);
    }
}

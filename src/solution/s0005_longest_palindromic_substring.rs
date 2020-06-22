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
    pub fn get_longest_palindrome_len(s: String) -> usize {
        let len = s.len();
        if len <= 1 {
            return 0;
        }
        let mut str = String::from("#");
        //enter #
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            str.push(chars[i]);
            str.push('#');
        }
        let after_chars: Vec<char> = str.chars().collect();

        let mut maxlen = 0;
        for i in 0..after_chars.len() {
            let mut cur_len = 0;
            let mut k = 1;

            while i >= k && i + k < after_chars.len() {
                if after_chars[i - k] == after_chars[i + k] {
                    cur_len += 2;
                } else {
                    break;
                }
                k += 1;
            }
            cur_len = cur_len / 2;
            if cur_len > maxlen {
                maxlen = cur_len;
            }
        }
        maxlen
    }
    pub fn longest_palindrome_ma(s: String) -> String {
        let len = s.len();
        if len <= 1 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let (mut cur_len, mut cur_start, mut cur_end) = (0, 0, 0);
        let mut maxlen = 0;
        let mut idx = 0;
        while idx < len {
            let (mut i, mut j) = (idx, idx);
            let ch = chars[idx];
            //all same char
            while i > 0 && ch == chars[i - 1] {
                i -= 1;
            }
            while j < len - 1 && ch == chars[j + 1] {
                j += 1;
            }
            idx = j + 1;
            while i > 0 && j < len - 1 && chars[i - 1] == chars[j + 1] {
                i -= 1;
                j += 1;
            }
            cur_len = j - i + 1;
            if cur_len > maxlen {
                maxlen = cur_len;
                cur_start = i;
                cur_end = j;
            }
        }
        s[cur_start..cur_end + 1].to_string()
    }
    // by dynamic program
    pub fn longest_palindrome_dp(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut table = vec![vec![false; len]; len];
        let mut result = chars[0].to_string();
        let (mut i, mut j) = (0, 0);
        for j in 0..len {
            table[j][j] = true;
            for i in 0..j {
                if chars[i] == chars[j] && ((j - i <= 1) || (table[i + 1][j - 1])) {
                    table[i][j] = true;
                    result = s[i..j + 1].to_string();
                }
            }
        }
        result
    }

    // by Manacher
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }

        let mut ss = String::from("#");
        let chars: Vec<char> = s.chars().collect();

        for i in 0..len {
            ss.push(chars[i]);
            ss.push('#');
        }

        let after_chars: Vec<char> = ss.chars().collect();
        let new_len = ss.len();

        let mut mx_right = 0;
        let mut mid_pos = 0;
        let mut maxlen = 0;
        let mut radius = vec![1; new_len];
        let mut maxi = 0;
        for i in 1..new_len {
            if mx_right > i {
                let j = 2 * mid_pos - i; //symmetrial point about mid_pos
                let boundary = mx_right - i; //boundary of mx-right
                if radius[j] > boundary {
                    radius[i] = boundary;
                } else {
                    radius[i] = radius[j];
                }
            } else {
                radius[i] = 1;
            }

            while i - radius[i] > 0
                && i + radius[i] < new_len
                && after_chars[i - radius[i]] == after_chars[i + radius[i]]
            {
                radius[i] += 1;
            }

            if i + radius[i] > mx_right {
                mx_right = i + radius[i];
                mid_pos = i;
            }

            if radius[i] - 1 > maxlen {
                maxlen = radius[i] - 1;
                maxi = i;
            }
        }

        if (maxi - maxlen) % 2 == 0 {
            s[(maxi - maxlen) / 2..(maxi + maxlen) / 2].to_string()
        } else {
            s[(maxi - maxlen - 1) / 2..(maxi + maxlen + 1) / 2].to_string()
        }
    }
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

/**
 * [91] Decode Ways
 *
 * A message containing letters from A-Z is being encoded to numbers using the following mapping:
 *
 *
 * 'A' -> 1
 * 'B' -> 2
 * ...
 * 'Z' -> 26
 *
 *
 * Given a non-empty string containing only digits, determine the total number of ways to decode it.
 *
 * Example 1:
 *
 *
 * Input: "12"
 * Output: 2
 * Explanation: It could be decoded as "AB" (1 2) or "L" (12).
 *
 *
 * Example 2:
 *
 *
 * Input: "226"
 * Output: 3
 * Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-ways/
// discuss: https://leetcode.com/problems/decode-ways/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let chars: Vec<char> = s.chars().collect();
        if chars[0] == '0' {
            return 0;
        }
        if s.len() == 1 && chars[0] != '0' {
            return 1;
        }

        let mut mem = vec![0; s.len() + 1];
        if chars[0] == '0' {
            mem[0] = 0;
        } else {
            mem[0] = 1;
        }
        if chars[1] == '0' {
            mem[1] = 0;
        } else {
            mem[1] = 1;
        }

        for i in 2..s.len() + 1 {
            let prev = chars[i - 1] as u32 - '0' as u32;
            if prev >= 1 && prev <= 9 {
                mem[i] += mem[i - 1];
            }

            let tail_1 = chars[i - 2] as u32 - '0' as u32;
            let tail_2 = chars[i - 1] as u32 - '0' as u32;
            let tail = tail_1 * 10 + tail_2;
            if tail >= 10 && tail <= 26 {
                mem[i] += mem[i - 2];
            }
        }
        return mem[mem.len() - 1];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_91() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
    }
}

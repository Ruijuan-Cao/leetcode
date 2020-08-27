/**
 * [76] Minimum Window Substring
 *
 * Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).
 *
 * Example:
 *
 *
 * Input: S = "ADOBECODEBANC", T = "ABC"
 * Output: "BANC"
 *
 *
 * Note:
 *
 *
 * 	If there is no such window in S that covers all characters in T, return the empty string "".
 * 	If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-window-substring/
// discuss: https://leetcode.com/problems/minimum-window-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn min_window1(s: String, t: String) -> String {
        if t.is_empty() || t.len() > s.len() {
            return "".to_string();
        }
        let t_len = t.len();

        let (mut left, mut right) = (0, 0);
        let mut count = 0;
        let mut min_len = std::usize::MAX;
        let mut result = (0, 0);
        let mut count_map = Solution::count_char(t);
        println!("{:?}", count_map);
        let chars: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            if count_map[&chars[i]] > 0 {
                count += 1;
            }
            let tmp = count_map[&chars[i]];
            count_map.entry(chars[left]).or_insert(tmp - 1);

            while count == t_len {
                if i - left + 1 < min_len {
                    min_len = i - left + 1;
                    result.0 = left;
                    result.1 = i;
                }
                let tmp = count_map[&chars[left]];
                count_map.entry(chars[left]).or_insert(tmp + 1);
                if count_map[&chars[left]] > 0 {
                    count -= 1;
                }
                left += 1;
            }
        }

        s[result.0..result.1].to_string()
    }

    fn count_char(s: String) -> HashMap<char, i32> {
        let mut res = HashMap::new();
        for ch in s.chars().into_iter() {
            *res.entry(ch).or_insert(0) += 1;
        }
        res
    }

    pub fn min_window(s: String, t: String) -> String {
        let mut mem = vec![0; 128];
        for ch in t.chars().into_iter() {
            mem[ch as usize] += 1;
        }
        let mut t_len = t.len();
        let mut left = 0;
        let mut min_left = 0;
        let mut min_right = std::usize::MAX;
        let chars: Vec<char> = s.chars().collect();
        for right in 0..s.len() {
            if mem[chars[right] as usize] > 0 {
                t_len -= 1;
            }
            mem[chars[right] as usize] -= 1;

            if t_len == 0 {
                while mem[chars[left] as usize] < 0 {
                    mem[chars[left] as usize] += 1;
                    left += 1;
                }
                if right - left < min_right - min_left {
                    min_left = left;
                    min_right = right;
                }
                mem[chars[left] as usize] += 1;
                left += 1;
                t_len += 1;
            }
        }
        if min_right == std::usize::MAX {
            "".to_string()
        } else {
            s[min_left..min_right + 1].to_string()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76() {
        assert_eq!(
            Solution::min_window1("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
    }
}

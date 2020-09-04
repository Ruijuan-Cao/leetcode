/**
 * [93] Restore IP Addresses
 *
 * Given a string containing only digits, restore it by returning all possible valid IP address combinations.
 *
 * Example:
 *
 *
 * Input: "25525511135"
 * Output: ["255.255.11.135", "255.255.111.35"]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/restore-ip-addresses/
// discuss: https://leetcode.com/problems/restore-ip-addresses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        if s.len() > 0 && s.len() < 4 || s.len() > 12 {
            return result;
        }

        Self::restore_ip(s, 4, 0, "".to_string(), &mut result);
        result
    }

    fn restore_ip(s: String, n: usize, index: usize, ip: String, result: &mut Vec<String>) {
        if n == 0 {
            if index == s.len() {
                result.push(ip);
            }
            return;
        }

        for i in index + 1..s.len() + 1 {
            let tmp = s[index..i].to_string();
            if Self::is_num(tmp.clone()) {
                if ip.len() == 0 {
                    Self::restore_ip(s.clone(), n - 1, i, tmp.clone(), result);
                } else {
                    Self::restore_ip(s.clone(), n - 1, i, ip.clone() + "." + &tmp, result);
                }
            } else {
                break;
            }
        }
    }

    fn is_num(num: String) -> bool {
        if num.len() > 3 {
            return false;
        }
        let chars: Vec<char> = num.chars().collect();
        let mut nums = Vec::new();
        for i in 0..num.len() {
            let ch = chars[i] as i32 - '0' as i32;
            if ch >= 0 && ch <= 9 {
                nums.push(ch);
            } else {
                return false;
            }
        }
        let mut sum = 0;
        for i in 0..nums.len() {
            sum = sum * 10 + nums[i];
        }
        if sum >= 0 && sum <= 255 {
            return true;
        } else {
            return false;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_93() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()]
        );
    }
}

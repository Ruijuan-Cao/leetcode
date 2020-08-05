/**
 * [65] Valid Number
 *
 * Validate if a given string can be interpreted as a decimal number.
 *
 * Some examples:<br />
 * "0" => true<br />
 * " 0.1 " => true<br />
 * "abc" => false<br />
 * "1 a" => false<br />
 * "2e10" => true<br />
 * " -90e3   " => true<br />
 * " 1e" => false<br />
 * "e3" => false<br />
 * " 6e-1" => true<br />
 * " 99e2.5 " => false<br />
 * "53.5e93" => true<br />
 * " --6 " => false<br />
 * "-+3" => false<br />
 * "95a54e53" => false
 *
 * Note: It is intended for the problem statement to be ambiguous. You should gather all requirements up front before implementing one. However, here is a list of characters that can be in a valid decimal number:
 *
 *
 * 	Numbers 0-9
 * 	Exponent - "e"
 * 	Positive/negative sign - "+"/"-"
 * 	Decimal point - "."
 *
 *
 * Of course, the context of these characters also matters in the input.
 *
 * Update (2015-02-10):<br />
 * The signature of the C++ function had been updated. If you still see your function signature accepts a const char * argument, please click the reload button to reset your code definition.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-number/
// discuss: https://leetcode.com/problems/valid-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// hope that regex was included in std library...
// TODO: NFA
impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut s = s.trim();
        println!("{:?}", s);
        let mut chars: Vec<char> = s.chars().collect();
        let mut digit = false;
        let mut dot = false;
        let mut signal = false;
        let mut e = false;
        let mut last = chars[0];

        for i in 0..chars.len() {
            if chars[i] >= '0' && chars[i] <= '9' {
                digit = true;
            } else if chars[i] == '.' {
                if digit == false {
                    return false;
                }
                if e {
                    return false;
                }
                dot = true;
            } else if chars[i] == '+' || chars[i] == '-' {
                if signal || dot {
                    return false;
                }
                if (digit || e) && last != 'e' {
                    return false;
                }
                signal = true;
            } else if chars[i] == 'e' {
                if !digit {
                    return false;
                }
                e = true;
            } else {
                return false;
            }
            last = chars[i];
        }
        if last == 'e' || last == '+' || last == '-' || last == '.' {
            return false;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_65() {
        assert_eq!(Solution::is_number("0".to_string()), true);
        assert_eq!(Solution::is_number(" 0.1".to_string()), true);
        assert_eq!(Solution::is_number("abc".to_string()), false);
        assert_eq!(Solution::is_number("1 a".to_string()), false);
        assert_eq!(Solution::is_number("2e10".to_string()), true);
        assert_eq!(Solution::is_number(" -90e3".to_string()), true);
        assert_eq!(Solution::is_number("1e".to_string()), false);
        assert_eq!(Solution::is_number("e3".to_string()), false);
        assert_eq!(Solution::is_number(" 6e-1".to_string()), true);
        assert_eq!(Solution::is_number(" 99e2.5".to_string()), false);
        assert_eq!(Solution::is_number("53.5e93".to_string()), true);
        assert_eq!(Solution::is_number(" --6 ".to_string()), false);
        assert_eq!(Solution::is_number("-+3".to_string()), false);
        assert_eq!(Solution::is_number("95a54e53".to_string()), false);
    }
}

/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 *
 * Example 1:
 *
 *
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 *
 * Example 2:
 *
 *
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *
 *
 * Note:
 *
 * <ol>
 * 	The length of both num1 and num2 is < 110.
 * 	Both num1 and num2 contain only digits 0-9.
 * 	Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 * 	You must not use any built-in BigInteger library or convert the inputs to integer directly.
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/multiply-strings/
// discuss: https://leetcode.com/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO
use std::char::from_digit;
use std::collections::VecDeque;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut num1: Vec<u32> = num1.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut num2: Vec<u32> = num2.chars().map(|c| c.to_digit(10).unwrap()).collect();

        // let mut result = Vec::new();
        let mut overflow = 0;
        num1.reverse();
        num2.reverse();

        let mut res = 0;
        let mut temp: VecDeque<u32> = VecDeque::with_capacity(num2.len() + 1);
        let mut n10 = 1;
        for (i, multiplier) in num1.iter().enumerate() {
            overflow = 0;
            for &multiplicand in num2.iter() {
                let num = multiplicand * multiplier + overflow;
                if num >= 10 {
                    overflow = num / 10;
                } else {
                    overflow = 0;
                }
                temp.push_back(num % 10);
            }
            if overflow > 0 {
                temp.push_back(overflow);
            }
            let mut temp_num = 0;
            while temp.len() > 0 {
                temp.pop_back()
                    .map(|digit| temp_num = temp_num * 10 + digit);
            }

            if n10 == 1 {
                res += temp_num;
            } else {
                res += temp_num * n10 as u32;
            }
            n10 *= 10;
        }

        res.to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6");
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088"
        );
    }
}

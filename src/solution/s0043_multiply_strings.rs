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
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut result = vec![0; num1.len() + num2.len()];
        for (i, ch1) in num1.chars().rev().enumerate() {
            for (j, ch2) in num2.chars().rev().enumerate() {
                let a = ch1.to_digit(10).unwrap();
                let b = ch2.to_digit(10).unwrap();

                let multi = a * b + result[i + j];
                result[i + j] = multi % 10;
                result[i + j + 1] += multi / 10;
            }
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }
        result.iter().rev().map(|s| s.to_string()).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        // assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6");
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088"
        );
        assert_eq!(
            Solution::multiply("123456789".to_string(), "987654321".to_string()),
            "121932631112635269"
        );
    }
}

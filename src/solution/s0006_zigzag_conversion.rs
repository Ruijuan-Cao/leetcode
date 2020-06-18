/**
 * [6] ZigZag Conversion
 *
 * The string <code>"PAYPALISHIRING"</code> is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: <code>"PAHNAPLSIIGYIR"</code>
 *
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 *
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/
// discuss: https://leetcode.com/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {}
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
        assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
    }
}

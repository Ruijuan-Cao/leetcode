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
    pub fn convert_complex(s: String, num_rows: i32) -> String {
        let len = s.len();

        if num_rows < 2 || len < 3 {
            return s;
        }
        let num_rows = num_rows as usize;
        let group = num_rows + num_rows - 2;
        let group_num = len / group;
        let group_re = len % group;
        let mut overflow = 0;
        if group_re > 0 {
            overflow = 1;
        }
        if group_re > num_rows {
            overflow += group_re - num_rows;
        }
        let column = (num_rows - 1) * group_num + overflow;
        let chars: Vec<char> = s.chars().collect();
        let mut new_chars = vec![vec!['#'; column]; num_rows as usize];
        let mut x = 0;
        let mut y = 0;
        let mut reverse = true;
        for i in 0..len {
            new_chars[x][y] = chars[i];
            if x < num_rows - 1 && reverse {
                x += 1;
            } else {
                reverse = false;
                if x > 0 {
                    x -= 1;
                }
                if y < column - 1 {
                    y += 1;
                }
            }
            if x == 0 {
                reverse = true;
            }
        }
        let mut result = String::from("");
        for i in 0..num_rows {
            for j in 0..column {
                if new_chars[i][j] != '#' {
                    result.push(new_chars[i][j]);
                }
            }
        }
        result
    }
    pub fn convert(s: String, num_rows: i32) -> String {
        let len = s.len();
        if num_rows < 2 || len < 3 {
            return s;
        }

        let num_rows = num_rows as usize;
        let group = num_rows + num_rows - 2;
        let group_num = len / group;
        let group_re = len % group;

        let chars: Vec<char> = s.chars().collect();
        let mut result = String::from("");
        for i in 0..num_rows {
            for j in 0..group_num + 1 {
                let index = i + j * group;
                if index < len {
                    result.push(chars[index]);
                }

                if i > 0 && i < num_rows - 1 {
                    let index = i + j * group + 2 * (num_rows - 1 - i);
                    if index < len {
                        result.push(chars[index]);
                    }
                }
            }
        }
        result
    }
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
        assert_eq!(Solution::convert("ABCDE".to_string(), 4), "ABCED");

        assert_eq!(Solution::convert("A".to_string(), 1), "A");
        assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
    }
}

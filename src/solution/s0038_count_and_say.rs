/**
 * [38] Count and Say
 *
 * The count-and-say sequence is the sequence of integers with the first five terms as following:
 *
 *
 * 1.     1
 * 2.     11
 * 3.     21
 * 4.     1211
 * 5.     111221
 *
 *
 * 1 is read off as "one 1" or 11.<br />
 * 11 is read off as "two 1s" or 21.<br />
 * 21 is read off as "one 2, then one 1" or 1211.
 *
 * Given an integer n where 1 <= n <= 30, generate the n^th term of the count-and-say sequence.
 *
 * Note: Each term of the sequence of integers will be represented as a string.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: 1
 * Output: "1"
 *
 *
 * Example 2:
 *
 *
 * Input: 4
 * Output: "1211"
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-and-say/
// discuss: https://leetcode.com/problems/count-and-say/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::char::from_digit;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = vec!['1'];
        for _ in 1..n {
            let mut temp = Vec::new();
            let mut i = 0;
            while i < result.len() {
                let mut j = i + 1;
                while j < result.len() && result[j] == result[i] {
                    j += 1;
                    continue;
                }
                let count = (j - i) as u32;
                temp.push(from_digit(count, 10).unwrap());
                temp.push(result[i]);
                i = j;
            }
            result = temp;
        }
        result.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}

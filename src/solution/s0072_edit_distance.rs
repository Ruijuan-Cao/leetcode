/**
 * [72] Edit Distance
 *
 * Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.
 *
 * You have the following 3 operations permitted on a word:
 *
 * <ol>
 * 	Insert a character
 * 	Delete a character
 * 	Replace a character
 * </ol>
 *
 * Example 1:
 *
 *
 * Input: word1 = "horse", word2 = "ros"
 * Output: 3
 * Explanation:
 * horse -> rorse (replace 'h' with 'r')
 * rorse -> rose (remove 'r')
 * rose -> ros (remove 'e')
 *
 *
 * Example 2:
 *
 *
 * Input: word1 = "intention", word2 = "execution"
 * Output: 5
 * Explanation:
 * intention -> inention (remove 't')
 * inention -> enention (replace 'i' with 'e')
 * enention -> exention (replace 'n' with 'x')
 * exention -> exection (replace 'n' with 'c')
 * exection -> execution (insert 'u')
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/edit-distance/
// discuss: https://leetcode.com/problems/edit-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![0i32; n + 1]; m + 1];
        // Self::helper(word1, word2, 0, 0, &mut memo)

        for i in 0..m + 1 {
            dp[i][0] = i as i32;
        }
        for i in 0..n + 1 {
            dp[0][i] = i as i32;
        }
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = i32::min(dp[i - 1][j - 1], i32::min(dp[i - 1][j], dp[i][j - 1])) + 1;
                }
            }
        }
        dp[m][n]
    }

    fn helper(
        word1: Vec<char>,
        word2: Vec<char>,
        i: usize,
        j: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if i == word1.len() {
            return word2.len() as i32 - 1;
        }
        if j == word2.len() {
            return word1.len() as i32 - 1;
        }
        if memo[i][j] > 0 {
            return memo[i][j] as i32;
        }

        let mut result = 0;
        if word1[i] == word2[j] {
            return Self::helper(word1, word2, i + 1, j + 1, memo);
        } else {
            let insert_counter = Self::helper(word1.clone(), word2.clone(), i, j + 1, memo);
            let delete_counter = Self::helper(word1.clone(), word2.clone(), i + 1, j, memo);
            let replace_counter = Self::helper(word1, word2, i + 1, j + 1, memo);
            result = i32::min(insert_counter, i32::min(delete_counter, replace_counter)) + 1;
            println!(
                "-----{}-{}-{}-{}",
                insert_counter, delete_counter, replace_counter, result
            );
            memo[i][j] = result;
            println!("{}-{}-{:?}", i, j, memo);
            return result;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_72() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}

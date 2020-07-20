/**
 * [52] N-Queens II
 *
 * The n-queens puzzle is the problem of placing n queens on an n&times;n chessboard such that no two queens attack each other.
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/8-queens.png" style="width: 258px; height: 276px;" />
 *
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 *
 * Example:
 *
 *
 * Input: 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown below.
 * [
 *  [".Q..",  // Solution 1
 *   "...Q",
 *   "Q...",
 *   "..Q."],
 *
 *  ["..Q.",  // Solution 2
 *   "Q...",
 *   "...Q",
 *   ".Q.."]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens-ii/
// discuss: https://leetcode.com/problems/n-queens-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let n = n as usize;
        let mut num = 0;
        let mut cur = vec![vec!['.'; n]; n];
        Self::solve(n, 0, &mut cur, &mut num);
        num
    }
    pub fn solve(n: usize, row: usize, cur: &mut Vec<Vec<char>>, num: &mut i32) {
        for col in 0..n {
            let mut valid = true;
            //check valid
            for i in 0..row {
                if cur[i][col] == 'Q' {
                    valid = false;
                    break;
                }
            }
            let mut i = row as i32 - 1;
            let mut j = col as i32 - 1;
            while i >= 0 && j >= 0 {
                if cur[i as usize][j as usize] == 'Q' {
                    valid = false;
                    break;
                }
                i -= 1;
                j -= 1;
            }

            let mut i = row as i32 - 1;
            let mut j = col + 1;
            while i >= 0 && j < n {
                if cur[i as usize][j] == 'Q' {
                    valid = false;
                    break;
                }
                i -= 1;
                j += 1;
            }

            if valid {
                cur[row][col] = 'Q';
                if row == n - 1 {
                    *num += 1;
                } else {
                    Self::solve(n, row + 1, cur, num);
                }
                cur[row][col] = '.';
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_52() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(8), 92);
        assert_eq!(Solution::total_n_queens(13), 73712);
        assert_eq!(Solution::total_n_queens(14), 365596);
    }
}

/**
 * [51] N-Queens
 *
 * The n-queens puzzle is the problem of placing n queens on an n*n chessboard such that no two queens attack each other.
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/10/12/8-queens.png" style="width: 258px; height: 276px;" />
 *
 * Given an integer n, return all distinct solutions to the n-queens puzzle.
 *
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space respectively.
 *
 * Example:
 *
 *
 * Input: 4
 * Output: [
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
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens/
// discuss: https://leetcode.com/problems/n-queens/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n <= 0 {
            return Vec::new();
        }
        let n = n as usize;
        let mut result = Vec::new();
        let mut cur = vec![vec!['.'; n]; n];
        Self::solve(n, 0, &mut cur, &mut result);
        result
    }
    pub fn solve(n: usize, row: usize, cur: &mut Vec<Vec<char>>, result: &mut Vec<Vec<String>>) {
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
                    result.push(cur.iter().map(|vec| vec.iter().collect()).collect());
                } else {
                    Self::solve(n, row + 1, cur, result);
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
    fn test_51() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
}

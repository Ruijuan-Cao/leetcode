/**
 * [73] Set Matrix Zeroes
 *
 * Given a m x n matrix, if an element is 0, set its entire row and column to 0. Do it <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>.
 *
 * Example 1:
 *
 *
 * Input:
 * [
 *   [1,1,1],
 *   [1,0,1],
 *   [1,1,1]
 * ]
 * Output:
 * [
 *   [1,0,1],
 *   [0,0,0],
 *   [1,0,1]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input:
 * [
 *   [0,1,2,0],
 *   [3,4,5,2],
 *   [1,3,1,5]
 * ]
 * Output:
 * [
 *   [0,0,0,0],
 *   [0,4,5,0],
 *   [0,3,1,0]
 * ]
 *
 *
 * Follow up:
 *
 *
 * 	A straight forward solution using O(mn) space is probably a bad idea.
 * 	A simple improvement uses O(m + n) space, but still not the best solution.
 * 	Could you devise a constant space solution?
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/set-matrix-zeroes/
// discuss: https://leetcode.com/problems/set-matrix-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        if m == 0 {
            return;
        }
        let n = matrix[0].len();
        if n == 0 {
            return;
        }
        let mut row0 = matrix[0][0];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    if i == 0 {
                        row0 = 0;
                    } else {
                        matrix[i][0] = 0;
                    }
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }
        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 0..m {
                    matrix[i][j] = 0;
                }
            }
        }
        if matrix[0][0] == 0 {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
        if row0 == 0 {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_73() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}

/**
 * [85] Maximal Rectangle
 *
 * Given a 2D binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
 *
 * Example:
 *
 *
 * Input:
 * [
 *   ["1","0","1","0","0"],
 *   ["1","0","1","1","1"],
 *   ["1","1","1","1","1"],
 *   ["1","0","0","1","0"]
 * ]
 * Output: 6
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-rectangle/
// discuss: https://leetcode.com/problems/maximal-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }
        let r = matrix.len();
        let c = matrix[0].len();
        let mut heights = vec![0; c + 1];
        let mut max_area = 0;
        for i in 0..r {
            for j in 0..c {
                if matrix[i][j] == '1' {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }
            let mut s = Vec::new();
            for idx in 0..c + 1 {
                while s.len() > 0 && heights[idx] < heights[s[s.len() - 1]] {
                    let h = heights[s[s.len() - 1]];
                    s.pop();
                    if s.len() > 0 {
                        max_area = i32::max(max_area, h * (idx - s[s.len() - 1] - 1) as i32);
                    }
                }
                s.push(idx);
            }
        }
        max_area
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_85() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }
}

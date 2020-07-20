/**
 * [54] Spiral Matrix
 *
 * Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.
 *
 * Example 1:
 *
 *
 * Input:
 * [
 *  [ 1, 2, 3 ],
 *  [ 4, 5, 6 ],
 *  [ 7, 8, 9 ]
 * ]
 * Output: [1,2,3,6,9,8,7,4,5]
 *
 *
 * Example 2:
 *
 * Input:
 * [
 *   [1, 2, 3, 4],
 *   [5, 6, 7, 8],
 *   [9,10,11,12]
 * ]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix/
// discuss: https://leetcode.com/problems/spiral-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        if matrix.len() < 1 {
            return result;
        }
        let (mut x_min, mut x_max, mut y_min, mut y_max) = (0, matrix.len(), 0, matrix[0].len());
        loop {
            for y in y_min..y_max {
                result.push(matrix[x_min][y]);
                println!("1:{}-{}-{:?}", x_min, y, result);
            }
            x_min += 1;
            if x_min == x_max {
                break;
            }

            for x in x_min..x_max {
                result.push(matrix[x][y_max - 1]);
                println!("2:{}-{}-{:?}", x, y_max - 1, result);
            }
            y_max -= 1;
            if y_min == y_max {
                break;
            }

            let mut y = y_max - 1;
            while y >= y_min {
                result.push(matrix[x_max - 1][y]);
                println!("3:{}-{}-{:?}", x_max - 1, y, result);
                y -= 1;
            }
            x_max -= 1;
            if x_min == x_max {
                break;
            }

            let mut x = x_max - 1;
            while x >= x_min {
                result.push(matrix[x][y_min]);
                println!("4:{}-{}-{:?}", x, y_min, result);
                x -= 1;
            }
            y_min += 1;
            if y_min == y_max {
                break;
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
    fn test_54() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3]]), vec![1, 2, 3]);
        assert_eq!(
            Solution::spiral_order(vec![vec![1], vec![2], vec![3],]),
            vec![1, 2, 3]
        );
        assert_eq!(Solution::spiral_order(vec![vec![1],]), vec![1]);
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2], vec![4, 5],]),
            vec![1, 2, 5, 4]
        );
    }
}

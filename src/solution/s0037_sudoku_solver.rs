/**
 * [37] Sudoku Solver
 *
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 *
 * A sudoku solution must satisfy all of the following rules:
 *
 * <ol>
 * 	Each of the digits 1-9 must occur exactly once in each row.
 * 	Each of the digits 1-9 must occur exactly once in each column.
 * 	Each of the the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
 * </ol>
 *
 * Empty cells are indicated by the character '.'.
 *
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px" /><br />
 * <small>A sudoku puzzle...</small>
 *
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png" style="height:250px; width:250px" /><br />
 * <small>...and its solution numbers marked in red.</small>
 *
 * Note:
 *
 *
 * 	The given board contain only digits 1-9 and the character '.'.
 * 	You may assume that the given Sudoku puzzle will have a single unique solution.
 * 	The given board size is always 9x9.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sudoku-solver/
// discuss: https://leetcode.com/problems/sudoku-solver/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/**
     * 回溯
     * 回溯解法：https://leetcode-cn.com/problems/sudoku-solver/solution/zi-cong-wo-xue-hui-liao-hui-su-suan-fa-zhong-yu-hu/
     * 逐行，从左到右，在每一个位置上试探1-9，成功就进入下一个位置，失败就取消本次选择，做下一个选择
     * 当前行试探完毕就换行，知道换到最后一行
     *
     * @param board
     */
    public void solution(char[][] board) {
        // 非法数独
        if (board == null || board.length != 9 || board[0] == null || board[0].length != 9)
            return;
        // 回溯法解决
        backTrace(board, 0, 0);
    }

    private boolean backTrace(char[][] board, int row, int col){
        int n = board.length; // 9
        // 当前行已全部试探过，换到下一行第一个位置
        if (col == 9)
            return backTrace(board, row + 1, 0);
        // 满足结束条件，全部行全部位置都已试探过
        if (row == n)
            // 最后一行最后一个位置[8][8]试探过后会试探[8][9]，会执行[9][0]，返回
            return true;
        // 这个位置数字已给出，不需要试探，直接试探下一个位置
        if (board[row][col] != '.')
            return backTrace(board, row, col + 1);
        // 遍历可选择列表(各选择之间并列)
        for (char c = '1'; c <= '9'; c++){
            // 排除不合法的选择
            if (!isValid(board, row, col, c))
                continue;
            // 做选择
            board[row][col] = c;
            // 进行下一步试探，发现当前选择能成功进行下去，就继续往下
            if (backTrace(board, row, col + 1))
                return true;
            // 撤销本次选择，并列进行下一次选择的试探
            board[row][col] = '.';
        }
        // 这个位置把1-9都试过了，都无法继续下去，说明上一次的选择失败，需要回溯
        return false;
    }
    
// TODO
impl Solution {
       pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        if board.len() != 9 || board[0].len()!=9   {
            return;
        }
        Solution::back_trace(board, 0,0);
    }
    pub fn back_trace(board:&Vec<Vec<char>>, row:usize, column:usize){
        
    }
     pub fn is_valid_sudoku(board: &Vec<Vec<char>>, row: usize, column: usize, ch: char) -> bool {
        for i in 0..board[0].len() {
            if board[row][i] == ch {
                return false;
            }
            if board[i][column] == ch {
                return false;
            }
            let i = (row/3)*3+(i/3);
            let j = (column/3)*3+(i%3);
            if board[i][j] == ch {
                return false;
            }
        }
        return true;
    }

}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_37() {
        let mut sudoku = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut sudoku);
        for i in 0..sudoku.len() {
            for j in 0..sudoku[0].len() {
                print!("{}  ", sudoku[i][j]);
            }
            println!("");
        }
        println!("----------------π");
        //Solution::is_valid_sudoku(sudoku);
        let mut sudoku = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut sudoku);
        for i in 0..sudoku.len() {
            for j in 0..sudoku[0].len() {
                print!("{}  ", sudoku[i][j]);
            }
            println!("");
        }
    }
}

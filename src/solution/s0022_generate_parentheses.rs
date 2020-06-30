/**
 * [22] Generate Parentheses
 *
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *
 *
 *
 * For example, given n = 3, a solution set is:
 *
 *
 * [
 *   "((()))",
 *   "(()())",
 *   "(())()",
 *   "()(())",
 *   "()()()"
 * ]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/generate-parentheses/
// discuss: https://leetcode.com/problems/generate-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;
// DFS
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return vec![];
        }
        let mut result = Vec::new();
        Solution::generate(n, n, &mut result, String::new());
        result
    }

    //generate
    pub fn generate(left: i32, right: i32, result: &mut Vec<String>, mut path: String) {
        if left > right {
            return;
        }
        if left == 0 && right == 0 {
            result.push(path);
            return;
        } else {
            if left > 0 {
                let new_path = path.clone();
                Solution::generate(left - 1, right, result, new_path + "(");
            }
            if right > 0 {
                Solution::generate(left, right - 1, result, path + ")");
            }
        }
    }

    //generate without recursive
    pub fn generate_parenthesis2(n: i32) -> Vec<String> {
        let mut result_set = HashSet::new();
        if n <= 0 {
            return vec![];
        } else if (n == 1) {
            return vec!["()".to_string()];
        } else {
            let mut haved = Solution::generate_parenthesis2(n - 1);
            for i in 0..haved.len() {
                // let mut cur_str = haved[i];
                // println!("cur_str={:?}", cur_str);
                for j in 0..haved[i].len() {
                    let chars: Vec<char> = haved[i].chars().collect();
                    if chars[j] == '(' {
                        // if &cur_str[j..j + 1] == "(" {
                        let cur_str_copy = haved[i].clone();
                        haved[i].insert(j + 1, '(');
                        haved[i].insert(j + 2, ')');
                        let xx = haved[i].clone();
                        result_set.insert(xx);
                        haved[i] = cur_str_copy;
                    }
                    let s = "()".to_string() + &haved[i];
                    result_set.insert(s);
                }
            }
        }
        let mut result: Vec<String> = Vec::new();
        for x in result_set.iter() {
            result.push(x.to_string());
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parenthesis2(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis2(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis2(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}

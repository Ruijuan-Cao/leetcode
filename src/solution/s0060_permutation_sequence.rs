/**
 * [60] Permutation Sequence
 *
 * The set [1,2,3,...,n] contains a total of n! unique permutations.
 *
 * By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
 *
 * <ol>
 * 	"123"
 * 	"132"
 * 	"213"
 * 	"231"
 * 	"312"
 * 	"321"
 * </ol>
 *
 * Given n and k, return the k^th permutation sequence.
 *
 * Note:
 *
 *
 * 	Given n will be between 1 and 9 inclusive.
 * 	Given k will be between 1 and n! inclusive.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 3, k = 3
 * Output: "213"
 *
 *
 * Example 2:
 *
 *
 * Input: n = 4, k = 9
 * Output: "2314"
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutation-sequence/
// discuss: https://leetcode.com/problems/permutation-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// target: split k = i! + j! + ...
use std::char::from_digit;
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut result = String::new();

        let mut nums: Vec<char> = "123456789".chars().collect();
        let mut fac = vec![1 as i32; n as usize];
        for i in 1..n {
            fac[i as usize] = fac[i as usize - 1] * i;
        }
        let mut k = k;
        k -= 1;
        for i in (1..n + 1).rev() {
            let j = k / fac[i as usize - 1];
            k %= fac[i as usize - 1];
            result.push(nums[j as usize]);
            nums.remove(j as usize);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_60() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        // assert_eq!(Solution::get_permutation(4, 9), "2314");
    }
}

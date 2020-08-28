/**
 * [56] Merge Intervals
 *
 * Given a collection of intervals, merge all overlapping intervals.
 *
 * Example 1:
 *
 *
 * Input: [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 *
 *
 * Example 2:
 *
 *
 * Input: [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-intervals/
// discuss: https://leetcode.com/problems/merge-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for an interval.
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}
impl Solution {
    pub fn merge(intervals: Vec<Interval>) -> Vec<Interval> {
        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval.start);

        let mut result: Vec<Interval> = Vec::new();
        for interval in intervals.into_iter() {
            match result.last_mut() {
                Some(last) => {
                    if last.end >= interval.start {
                        if interval.end > last.end {
                            last.end = interval.end;
                        }
                        continue;
                    }
                }
                None => {}
            }
            result.push(interval);
        }
        result
    }

    pub fn merge2(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a.cmp(b));

        let mut result: Vec<Vec<i32>> = Vec::new();
        for interval in intervals.into_iter() {
            match result.last_mut() {
                Some(last) => {
                    if last[1] >= interval[0] {
                        if interval[1] > last[1] {
                            last[1] = interval[1];
                        }
                        continue;
                    }
                }
                None => {}
            }
            result.push(interval);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_56() {
        assert_eq!(
            Solution::merge(vec![
                Interval::new(1, 3),
                Interval::new(2, 6),
                Interval::new(8, 10),
                Interval::new(15, 18)
            ]),
            vec![
                Interval::new(1, 6),
                Interval::new(8, 10),
                Interval::new(15, 18)
            ]
        );

        assert_eq!(
            Solution::merge2(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );

        assert_eq!(
            Solution::merge2(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }
}

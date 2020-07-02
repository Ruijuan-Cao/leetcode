/**
 * [30] Substring with Concatenation of All Words
 *
 * You are given a string, s, and a list of words, words, that are all of the same length. Find all starting indices of substring(s) in s that is a concatenation of each word in words exactly once and without any intervening characters.
 *
 * Example 1:
 *
 *
 * Input:
 *   s = "barfoothefoobarman",
 *   words = ["foo","bar"]
 * Output: [0,9]
 * Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
 * The output order does not matter, returning [9,0] is fine too.
 *
 *
 * Example 2:
 *
 *
 * Input:
 *   s = "wordgoodgoodgoodbestword",
 *   words = ["word","good","best","word"]
 * Output: []
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/substring-with-concatenation-of-all-words/
// discuss: https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
struct Term {
    expect: i32,
    count: i32,
}
impl Term {
    fn new(expect: i32, count: i32) -> Self {
        Term { expect, count }
    }
    fn inc_expect(&mut self) {
        self.expect += 1;
    }
    fn inc(&mut self) {
        self.count += 1;
    }
    fn dec(&mut self) {
        self.count -= 1;
    }
    fn exhausted(&self) -> bool {
        self.count > self.expect
    }
    fn reset(&mut self) {
        self.count = 0;
    }
}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    fn find_substring2(s: String, words: Vec<String>) -> Vec<i32> {
        let mut start_indices = Vec::<i32>::new();
        if words.is_empty() {
            return start_indices;
        }

        let word_size = words[0].len();
        let window_size = word_size * words.len();

        if let Some(last_split) = s.len().checked_sub(window_size) {
            let mut word_set = HashMap::with_capacity(words.len());
            words.iter().for_each(|w| {
                let counter = word_set.entry(&w[..]).or_insert(0);
                *counter += 1;
            });
            word_set.shrink_to_fit();
            let mut seen = word_set.keys().map(|k| (*k, 0)).collect::<HashMap<_, _>>();
            for i in 0..word_size.min(last_split + 1) {
                let mut j = i + window_size;
                while j <= s.len() {
                    let mut k = 1;
                    while k <= words.len() {
                        let current_pos = j - k * word_size;
                        let current = &s[current_pos..(current_pos + word_size)];
                        match seen.entry(current) {
                            Entry::Occupied(entry) => {
                                let res = entry.into_mut();
                                *res += 1;
                                if *res > *word_set.get(current).unwrap() {
                                    break;
                                } else {
                                    k += 1;
                                }
                            }
                            Entry::Vacant(_) => {
                                break;
                            }
                        }
                    }

                    let start = j - window_size;
                    if k > words.len() {
                        start_indices.push(start as i32);
                        j += word_size;
                    } else {
                        j = window_size + (j - (k - 1) * word_size);
                    }
                    seen.values_mut().for_each(|v| *v = 0);
                }
            }
        }
        start_indices
    }

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.len() < 1 {
            return vec![];
        }
        let word_len = words[0].len();
        let word_size = words.len();
        let all_len = word_len * word_size;
        let mut result: Vec<i32> = Vec::new();

        if s.len() - all_len > 0 {
            //init all word and count with 1
            let mut map: HashMap<&str, i32> = HashMap::with_capacity(word_size);
            words.iter().for_each(|w| {
                let counter = map.entry(&w[..]).or_insert(0);
                *counter += 1;
            });
            //another map, counter = 0
            let mut seen: HashMap<&str, i32> = map.keys().map(|k| (*k, 0)).collect();
            for i in 0..word_len.min(s.len() - all_len + 1) {
                let mut j = i + all_len;
                while j <= s.len() {
                    let mut k = 1;
                    while k <= word_size {
                        let cur_pos = j - k * word_len;
                        let cur = &s[cur_pos..(cur_pos + word_len)];
                        println!("{:?}-{}", cur, cur_pos);
                        match seen.entry(cur) {
                            Entry::Occupied(entry) => {
                                let res = entry.into_mut();
                                *res += 1;
                                let map_counter = *map.get(cur).unwrap();
                                println!("{}-{}", *res, map_counter);
                                if *res > map_counter {
                                    break;
                                } else {
                                    k += 1;
                                }
                            }
                            Entry::Vacant(_) => {
                                break;
                            }
                        }
                    }

                    let start = j - all_len;
                    if k > word_size {
                        result.push(start as i32);
                        j += word_len;
                    } else {
                        j = all_len + (j - (k - 1) * word_len);
                    }
                    seen.values_mut().for_each(|v| *v = 0);
                }
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
    fn test_30() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                "xxwordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![10]
        );
    }
}

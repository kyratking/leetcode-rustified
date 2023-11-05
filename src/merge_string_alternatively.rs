// Problem
// https://leetcode.com/problems/merge-strings-alternately

fn main() {}

fn merge_alternately(word1: String, word2: String) -> String {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let mut result: Vec<char> = Vec::new();
    let mut i = 0;
    while i < word1.len() || i < word2.len() {
        match word1.get(i) {
            Some(value) => result.push(*value as char),
            None => {}
        }
        match word2.get(i) {
            Some(value) => result.push(*value as char),
            None => {}
        }
    }
    return result.iter().collect();
}
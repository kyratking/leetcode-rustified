// Problem
// https://leetcode.com/problems/number-of-good-pairs/

use std::collections::HashMap;

fn main() {}
fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut map: HashMap<String, bool> = HashMap::new();
    for (idx, value) in nums.iter().enumerate() {
        for j in (idx + 1)..nums.len() {
            if (*value == nums[j]) {
                let pair = String::from(format!("{},{}", *value, nums[j]));
                if map.contains_key(&pair) {
                    count += 1;
                } else {
                    map.insert(pair, true);
                }
            }
        }
    }
    return count + map.len() as i32;
}

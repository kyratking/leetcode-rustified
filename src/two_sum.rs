// Problem
// https://leetcode.com/problems/two-sum/description/

use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    two_sum(nums, 9);
    println!("Hello, world!");
}

fn two_sum_memoized(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut memo = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;
        if memo.contains_key(&diff) {
            return vec![memo[&diff], i as i32];
        }
        memo.insert(diff, i as i32);
    }
    vec![]
}

fn two_sum_gpt(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut memo: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&j) = memo.get(&diff) {
            return vec![j as i32, i as i32];
        }
        memo.insert(*num, i);
    }
    vec![]
}

// O(n^2)
fn two_sum_approch_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if (nums[i] + nums[j]) == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

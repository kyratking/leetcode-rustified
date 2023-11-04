// Problem
// https://leetcode.com/problems/concatenation-of-array/description/

fn main() {}

fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    Vec::from(vec![Vec::from(nums.clone()), Vec::from(nums)]).concat()
}

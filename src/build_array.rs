// Problem
// https://leetcode.com/problems/build-array-from-permutation/

fn main() {
    build_array(vec![1, 2, 3, 4]);
}

fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for (index, value) in nums.iter().enumerate() {
        result.push(nums[nums[index] as usize]);
    }
    return result;
}

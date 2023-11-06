// Problem
// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies

fn main() {}

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    // 2 <= n <= 100
    let max: i32 = *(candies.iter().max().unwrap());
    let mut result: Vec<bool> = vec![];
    for i in 0..candies.len() {
        if candies[i] + extra_candies >= max {
            result.push(true);
        } else {
            result.push(false);
        }
    }
    return vec![];
}

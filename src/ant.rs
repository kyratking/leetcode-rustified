// Problem
// https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/description/

fn main() {
    println!("{}", get_last_moment(9, vec![], vec![]));
}

fn get_last_moment(scale: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    let max_left: f32 = if left.len() == 0 {
        NEG_INFINITY
    } else {
        left.iter().max().cloned().unwrap_or(0) as f32
    };
    let max_right: f32 = if right.len() == 0 {
        INFINITY
    } else {
        right.iter().min().cloned().unwrap_or(0) as f32
    };

    let scale: i32 = scale + 1;
    if scale - max_right as i32 > max_left as i32 {
        (scale - max_right as i32 - 1) as i32
    } else {
        max_left as i32
    }
}
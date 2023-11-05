// Problem
// https://leetcode.com/problems/find-the-winner-of-an-array-game

fn main() {}

// Highly unoptimized, takes 1200+ ms to complete whereas optimal is 7ms
fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    if arr.len() <= k as usize {
        return arr.iter().max().cloned().unwrap_or_default();
    }
    let mut arr = arr;
    let mut wins = 0;
    let mut winner = arr[0]; // access 0 directly due to constraint 2 <= arr.len() <= 10^5, otherwise, do error handling
    for i in 0..arr.len() {
        if wins == k {
            return winner;
        }
        if (arr[0] > arr[1]) {
            let removed = arr.remove(1);
            arr.push(removed);
            if winner != arr[0] {
                winner = arr[0];
                wins = 1;
            } else {
                wins += 1;
            }
        } else {
            let removed = arr.remove(0);
            arr.push(removed);
            if winner != arr[0] {
                winner = arr[0];
                wins = 1;
            } else {
                wins += 1;
            }
        }
    }
    return winner;
}

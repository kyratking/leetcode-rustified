use std::{collections::HashMap, time::Instant};

fn main() {
    let n = 184; // Change this to the desired value of n
    let mut memo: HashMap<i128, i128> = HashMap::new();
    let start = Instant::now();
    let result = fib(n, &mut memo);
    let duration = start.elapsed();
    println!("Fib({}) = {}", n, result);
    println!("Time Duration: {:?}", duration);
}

fn fib(n: i128, memo: &mut HashMap<i128, i128>) -> i128 {
    if n == 0 || n == 1 {
        memo.insert(n, n);
        return n;
    }

    if !memo.contains_key(&(n - 1)) {
        let fib_n_1 = fib(n - 1, memo);
        memo.insert(n - 1, fib_n_1);
    }

    if !memo.contains_key(&(n - 2)) {
        let fib_n_2 = fib(n - 2, memo);
        memo.insert(n - 2, fib_n_2);
    }

    return memo[&(n - 1)] + memo[&(n - 2)];
}

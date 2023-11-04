// Problem
// https://leetcode.com/problems/roman-to-integer/description/

use std::collections::HashMap;

fn main() {
    roman_to_int(String::from("Hellooo"));
}

fn roman_to_int(s: String) -> i32 {
    let mut map = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);
    let mut sum: i32 = 0;
    let mut last_val: i32 = 0;

    for (c) in s.chars().rev() {
        let value = map.get(&c);
        if let Some(&value) = map.get(&c) {
            sum += if value < last_val { -value } else { value };
            last_val = value;
        }
    }

    return sum;
}

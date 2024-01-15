// Problem
// https://www.codewars.com/kata/570523c146edc287a50014b1

fn number_joy(n: u32) -> bool {
    let sum: u32 = n
        .to_string()
        .chars()
        .fold(0, |acc, c| c.to_digit(10).unwrap_or(0) + acc);
    let reversed: u32 = sum
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap();
    if sum == reversed {
        return true;
    }
    return false;
}

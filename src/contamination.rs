// Problem
// https://www.codewars.com/kata/596fba44963025c878000039

fn contamination(text: &str, character: &str) -> String {
    if text == "" || character == "" {
        return String::from("");
    }
    let mut result: String = String::from("");
    for i in 0..text.len() {
        result.push(character.chars().nth(0).unwrap());
    }
    return result;
}

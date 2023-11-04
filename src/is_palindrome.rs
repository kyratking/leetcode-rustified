fn main() {
    let num = 112211;
    let num2 = 12312;
    let num3 = 12342;
    println!("Number: {}, isPalindrome: {}", num, is_palindrome(num));
    println!("Number: {}, isPalindrome: {}", num2, is_palindrome(num2));
    println!("Number: {}, isPalindrome: {}", num3, is_palindrome(num3));
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let log_of_x = (x as f32).log10().ceil() as i32;
    let mut copy = x;
    let mut remainder = 0;
    let mut reverse = 0;
    while copy != 0 {
        remainder = copy % 10;
        reverse = (reverse * 10) + remainder;
        copy -= remainder;
        copy /= 10;
    }
    if reverse == x {
        return true;
    }
    return false;
}

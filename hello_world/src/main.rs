fn isPalindrome(self, x: i32) -> bool {
    if x < 0 {
        false
    }
    let mut old_x = x;
    let mut num = 0;
    while x {
        let mut last_digit: i32 = x % 10;
        num = num * 10 + last_digit;
        x = x / 10;
    }
    num == old_x
}
// #Medium #Top_Interview_Questions #String #2024_09_04_Time_0_ms_(100.00%)_Space_2.1_MB_(82.19%)

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut chars = str.chars().peekable();
        let mut i = 0;
        let mut negative_sign = false;
        let mut num: i32 = 0;

        // Skip leading whitespaces
        while let Some(&ch) = chars.peek() {
            if ch == ' ' {
                chars.next();
            } else {
                break;
            }
        }

        // Handle sign
        if let Some(&ch) = chars.peek() {
            if ch == '+' {
                chars.next();
            } else if ch == '-' {
                chars.next();
                negative_sign = true;
            }
        }

        // Convert characters to integer
        while let Some(&ch) = chars.peek() {
            if ch >= '0' && ch <= '9' {
                chars.next();
                let digit = ch.to_digit(10).unwrap() as i32;
                let digit = if negative_sign { -digit } else { digit };

                // Check for overflow/underflow
                if num > i32::MAX / 10 || (num == i32::MAX / 10 && digit > 7) {
                    return i32::MAX;
                }
                if num < i32::MIN / 10 || (num == i32::MIN / 10 && digit < -8) {
                    return i32::MIN;
                }

                num = num * 10 + digit;
            } else {
                break;
            }
        }

        num
    }
}

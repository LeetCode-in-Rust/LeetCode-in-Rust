// #Easy #Math #Udemy_Integers #2024_09_04_Time_0_ms_(100.00%)_Space_2_MB_(83.29%)

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Negative numbers are not palindromes
        if x < 0 {
            return false;
        }

        let mut rev = 0;
        let mut local_x = x;

        // Reverse the integer
        while local_x > 0 {
            rev = rev * 10 + local_x % 10;
            local_x /= 10;
        }

        // Check if the reversed integer is the same as the original
        rev == x
    }
}

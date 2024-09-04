// #Medium #Top_Interview_Questions #Math #Udemy_Integers
// #2024_09_04_Time_0_ms_(100.00%)_Space_2.1_MB_(18.54%)

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut rev: i64 = 0; // Use i64 to handle overflow

        while x != 0 {
            rev = rev * 10 + (x % 10) as i64;
            x /= 10;
        }

        if rev > i32::MAX as i64 || rev < i32::MIN as i64 {
            return 0;
        }

        rev as i32
    }
}

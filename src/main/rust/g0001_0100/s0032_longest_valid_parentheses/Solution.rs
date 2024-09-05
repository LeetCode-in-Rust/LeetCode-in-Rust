// #Hard #Top_100_Liked_Questions #String #Dynamic_Programming #Stack #Big_O_Time_O(n)_Space_O(1)
// #2024_09_05_Time_0_ms_(100.00%)_Space_2.3_MB_(61.46%)

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_len = 0;
        let mut left = 0;
        let mut right = 0;
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();

        // Left to right pass
        for i in 0..n {
            if chars[i] == '(' {
                left += 1;
            } else {
                right += 1;
            }

            if right > left {
                left = 0;
                right = 0;
            }

            if left == right {
                max_len = max_len.max(left + right);
            }
        }

        // Right to left pass
        left = 0;
        right = 0;
        for i in (0..n).rev() {
            if chars[i] == '(' {
                left += 1;
            } else {
                right += 1;
            }

            if left > right {
                left = 0;
                right = 0;
            }

            if left == right {
                max_len = max_len.max(left + right);
            }
        }

        max_len as i32
    }
}

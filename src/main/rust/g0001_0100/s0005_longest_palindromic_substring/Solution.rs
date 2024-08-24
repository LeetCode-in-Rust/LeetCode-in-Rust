// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Dynamic_Programming
// #Data_Structure_II_Day_9_String #Algorithm_II_Day_14_Dynamic_Programming
// #Dynamic_Programming_I_Day_17 #Udemy_Strings #Big_O_Time_O(n)_Space_O(n)
// #2024_08_24_Time_1_ms_(92.60%)_Space_2.2_MB_(20.49%)

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        if n == 0 {
            return String::new();
        }

        // Step 1: Transform the string to avoid even/odd length issues
        let mut new_str = Vec::with_capacity(n * 2 + 1);
        new_str.push('#');
        for c in s.chars() {
            new_str.push(c);
            new_str.push('#');
        }

        let m = new_str.len();
        let mut dp = vec![0; m];
        let mut friend_center = 0;
        let mut friend_radius = 0;
        let mut lps_center = 0;
        let mut lps_radius = 0;

        // Step 2: Apply Manacher's Algorithm
        for i in 0..m {
            dp[i] = if friend_center + friend_radius > i {
                dp[2 * friend_center - i].min(friend_center + friend_radius - i)
            } else {
                1
            };

            while i + dp[i] < m && i >= dp[i] && new_str[i + dp[i]] == new_str[i - dp[i]] {
                dp[i] += 1;
            }

            if friend_center + friend_radius < i + dp[i] {
                friend_center = i;
                friend_radius = dp[i];
            }

            if lps_radius < dp[i] {
                lps_center = i;
                lps_radius = dp[i];
            }
        }

        // Step 3: Extract the longest palindromic substring
        let start = (lps_center - lps_radius + 1) / 2;
        let end = (lps_center + lps_radius - 1) / 2;
        s[start..end].to_string()
    }
}

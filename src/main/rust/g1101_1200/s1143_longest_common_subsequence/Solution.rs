// #Medium #Top_100_Liked_Questions #String #Dynamic_Programming
// #Algorithm_II_Day_17_Dynamic_Programming #Dynamic_Programming_I_Day_19
// #Udemy_Dynamic_Programming #Big_O_Time_O(n*m)_Space_O(n*m)
// #2024_09_13_Time_4_ms_(78.99%)_Space_5.8_MB_(55.07%)

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();

        // Create a 2D dp array initialized to 0
        let mut dp = vec![vec![0; m + 1]; n + 1];

        // Convert the strings to a vector of chars for easier access
        let chars1: Vec<char> = text1.chars().collect();
        let chars2: Vec<char> = text2.chars().collect();

        // Fill the dp array
        for i in 1..=n {
            for j in 1..=m {
                if chars1[i - 1] == chars2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        dp[n][m]
    }
}

// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Dynamic_Programming #Math
// #Combinatorics #Algorithm_II_Day_13_Dynamic_Programming #Dynamic_Programming_I_Day_15
// #Level_1_Day_11_Dynamic_Programming #Big_O_Time_O(m*n)_Space_O(m*n)
// #2024_09_07_Time_0_ms_(100.00%)_Space_2.1_MB_(22.76%)

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];

        // Initialize the first row and first column
        for i in 0..m {
            dp[i][0] = 1;
        }
        for j in 0..n {
            dp[0][j] = 1;
        }

        // Fill the DP table
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m - 1][n - 1]
    }
}

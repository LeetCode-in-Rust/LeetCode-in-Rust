// #Medium #Array #Dynamic_Programming #Backtracking #Big_O_Time_O(n*(sum+s))_Space_O(n*(sum+s))
// #2024_09_12_Time_3_ms_(83.33%)_Space_2.2_MB_(41.67%)

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut sum: i32 = nums.iter().sum();
        let s = s.abs();
        
        // Invalid cases
        if s > sum || (sum + s) % 2 != 0 {
            return 0;
        }

        let target = (sum + s) / 2;
        let n = nums.len();

        // dp[i] represents the number of ways to form sum i
        let mut dp = vec![vec![0; n + 1]; (target + 1) as usize];
        dp[0][0] = 1;

        // Handle empty knapsack condition
        for i in 0..n {
            if nums[i] == 0 {
                dp[0][i + 1] = dp[0][i] * 2;
            } else {
                dp[0][i + 1] = dp[0][i];
            }
        }

        // Fill the dp table
        for i in 1..=target {
            for j in 0..n {
                dp[i as usize][j + 1] += dp[i as usize][j];
                if nums[j] <= i {
                    dp[i as usize][j + 1] += dp[(i - nums[j]) as usize][j];
                }
            }
        }

        dp[target as usize][n]
    }
}

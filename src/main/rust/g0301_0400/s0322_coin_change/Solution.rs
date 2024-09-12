// #Medium #Top_100_Liked_Questions #Array #Dynamic_Programming #Breadth_First_Search
// #Algorithm_II_Day_18_Dynamic_Programming #Dynamic_Programming_I_Day_20
// #Level_2_Day_12_Dynamic_Programming #Big_O_Time_O(m*n)_Space_O(amount)
// #2024_09_12_Time_3_ms_(100.00%)_Space_2.2_MB_(69.95%)

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![0; (amount + 1) as usize];
        dp[0] = 1;  // Base case: one way to make 0 amount (using no coins)

        for &coin in &coins {
            for i in coin..=amount {
                let prev = dp[(i - coin) as usize];
                if prev > 0 {
                    if dp[i as usize] == 0 {
                        dp[i as usize] = prev + 1;
                    } else {
                        dp[i as usize] = dp[i as usize].min(prev + 1);
                    }
                }
            }
        }
        
        // If the dp[amount] is still 0, it means no solution was found
        if dp[amount as usize] == 0 {
            -1
        } else {
            dp[amount as usize] - 1
        }
    }
}

// #Medium #Top_100_Liked_Questions #String #Dynamic_Programming
// #Algorithm_II_Day_18_Dynamic_Programming #Dynamic_Programming_I_Day_19
// #Udemy_Dynamic_Programming #Big_O_Time_O(n^2)_Space_O(n2)
// #2024_09_07_Time_0_ms_(100.00%)_Space_2.2_MB_(87.38%)

impl Solution {
    pub fn min_distance(w1: String, w2: String) -> i32 {
        let n1 = w1.len();
        let n2 = w2.len();

        // Ensure the longer word is `w1`
        if n2 > n1 {
            return Solution::min_distance(w2, w1);
        }

        let w1_chars: Vec<char> = w1.chars().collect();
        let w2_chars: Vec<char> = w2.chars().collect();

        // Initialize dp array
        let mut dp = vec![0; n2 + 1];
        for j in 0..=n2 {
            dp[j] = j as i32;
        }

        // Dynamic programming to calculate minimum distance
        for i in 1..=n1 {
            let mut pre = dp[0];
            dp[0] = i as i32;

            for j in 1..=n2 {
                let tmp = dp[j];
                dp[j] = if w1_chars[i - 1] != w2_chars[j - 1] {
                    1 + i32::min(pre, i32::min(dp[j], dp[j - 1]))
                } else {
                    pre
                };
                pre = tmp;
            }
        }

        dp[n2]
    }
}

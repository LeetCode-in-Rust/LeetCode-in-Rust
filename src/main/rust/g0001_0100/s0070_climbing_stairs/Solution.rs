// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #Dynamic_Programming #Math #Memoization
// #Algorithm_I_Day_12_Dynamic_Programming #Dynamic_Programming_I_Day_2
// #Level_1_Day_10_Dynamic_Programming #Udemy_Dynamic_Programming #Big_O_Time_O(n)_Space_O(n)
// #2024_09_07_Time_0_ms_(100.00%)_Space_2.1_MB_(16.87%)

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let n = n as usize;
        let mut cache = vec![0; n];

        // Base cases
        cache[0] = 1;
        cache[1] = 2;

        // Fill the cache using dynamic programming
        for i in 2..n {
            cache[i] = cache[i - 1] + cache[i - 2];
        }

        cache[n - 1]
    }
}

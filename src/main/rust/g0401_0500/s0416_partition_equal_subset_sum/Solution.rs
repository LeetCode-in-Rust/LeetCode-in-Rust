// #Medium #Top_100_Liked_Questions #Array #Dynamic_Programming #Level_2_Day_13_Dynamic_Programming
// #Big_O_Time_O(n*sums)_Space_O(n*sums) #2024_09_12_Time_20_ms_(74.07%)_Space_2.2_MB_(81.48%)

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sums: i32 = nums.iter().sum();
        
        if sums % 2 != 0 {
            return false;
        }
        
        sums /= 2;
        let mut dp = vec![false; (sums + 1) as usize];
        dp[0] = true;
        
        for &num in &nums {
            for sum in (num..=sums).rev() {
                dp[sum as usize] = dp[sum as usize] || dp[(sum - num) as usize];
            }
        }
        
        dp[sums as usize]
    }
}

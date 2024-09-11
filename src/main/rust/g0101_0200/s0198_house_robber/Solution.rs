// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Dynamic_Programming
// #Algorithm_I_Day_12_Dynamic_Programming #Dynamic_Programming_I_Day_3
// #Level_2_Day_12_Dynamic_Programming #Udemy_Dynamic_Programming #Big_O_Time_O(n)_Space_O(n)
// #2024_09_10_Time_0_ms_(100.00%)_Space_2.1_MB_(31.95%)

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }

        let mut profit = vec![0; nums.len()];
        profit[0] = nums[0];
        profit[1] = std::cmp::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            profit[i] = std::cmp::max(profit[i - 1], nums[i] + profit[i - 2]);
        }

        profit[nums.len() - 1]
    }
}

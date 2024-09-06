// #Medium #Top_100_Liked_Questions #Array #Dynamic_Programming #Greedy
// #Algorithm_II_Day_13_Dynamic_Programming #Dynamic_Programming_I_Day_4
// #Big_O_Time_O(n)_Space_O(1) #2024_09_06_Time_0_ms_(100.00%)_Space_2.1_MB_(95.65%)

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut length = 0;
        let mut max_length = 0;
        let mut min_jump = 0;

        for i in 0..(nums.len() - 1) {
            length -= 1;
            max_length -= 1;
            max_length = max_length.max(nums[i]);

            if length <= 0 {
                length = max_length;
                min_jump += 1;
            }

            if length >= (nums.len() - i - 1) as i32 {
                return min_jump;
            }
        }

        min_jump
    }
}

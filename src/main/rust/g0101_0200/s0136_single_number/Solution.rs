// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #Array #Bit_Manipulation
// #Data_Structure_II_Day_1_Array #Algorithm_I_Day_14_Bit_Manipulation #Udemy_Integers
// #Big_O_Time_O(N)_Space_O(1) #2024_09_09_Time_0_ms_(100.00%)_Space_2.2_MB_(45.48%)

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for num in nums {
            res ^= num;
        }
        res
    }
}

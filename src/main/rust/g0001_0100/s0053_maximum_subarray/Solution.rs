// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Dynamic_Programming
// #Divide_and_Conquer #Data_Structure_I_Day_1_Array #Dynamic_Programming_I_Day_5
// #Udemy_Famous_Algorithm #Big_O_Time_O(n)_Space_O(1)
// #2024_09_06_Time_7_ms_(89.94%)_Space_3.3_MB_(6.92%)

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut maxi = i32::MIN;
        let mut sum = 0;
        for &num in nums.iter() {
            // calculating sub-array sum
            sum += num;
            maxi = maxi.max(sum);
            // if the sum is negative, reset it to 0
            if sum < 0 {
                sum = 0;
            }
        }
        maxi
    }
}

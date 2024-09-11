// #Easy #Top_100_Liked_Questions #Array #Two_Pointers #Algorithm_I_Day_3_Two_Pointers
// #Programming_Skills_I_Day_6_Array #Udemy_Arrays #Big_O_Time_O(n)_Space_O(1)
// #2024_09_11_Time_0_ms_(100.00%)_Space_2.3_MB_(81.98%)

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut first_zero = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                Solution::swap(first_zero, i, nums);
                first_zero += 1;
            }
        }
    }

    fn swap(index1: usize, index2: usize, nums: &mut Vec<i32>) {
        nums.swap(index1, index2);
    }
}

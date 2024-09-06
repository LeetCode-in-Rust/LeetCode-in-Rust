// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Dynamic_Programming #Greedy
// #Algorithm_II_Day_12_Dynamic_Programming #Dynamic_Programming_I_Day_4 #Udemy_Arrays
// #Big_O_Time_O(n)_Space_O(1) #2024_09_06_Time_0_ms_(100.00%)_Space_2.2_MB_(49.50%)

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let sz = nums.len();
        let mut tmp = 1;

        for i in 0..sz {
            tmp -= 1;
            if tmp < 0 {
                return false;
            }
            // Update `tmp` to be the maximum value between current `tmp` and `nums[i]`.
            tmp = tmp.max(nums[i]);

            // Early exit: if `i + tmp` can reach or exceed the last index, return true.
            if i + tmp as usize >= sz - 1 {
                return true;
            }
        }
        true
    }
}

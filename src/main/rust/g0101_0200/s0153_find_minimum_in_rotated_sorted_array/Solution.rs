// #Medium #Top_100_Liked_Questions #Array #Binary_Search #Algorithm_II_Day_2_Binary_Search
// #Binary_Search_I_Day_12 #Udemy_Binary_Search #Big_O_Time_O(log_N)_Space_O(log_N)
// #2024_09_09_Time_1_ms_(77.10%)_Space_2.1_MB_(96.95%)

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left:isize = 0;
        let mut right:isize = nums.len() as isize - 1;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < nums[right as usize] {
                right = mid;
            } else {
                left = mid;
            }
        }

        if nums[left as usize] < nums[right as usize] {
            nums[left as usize]
        } else {
            nums[right as usize]
        }
    }
}

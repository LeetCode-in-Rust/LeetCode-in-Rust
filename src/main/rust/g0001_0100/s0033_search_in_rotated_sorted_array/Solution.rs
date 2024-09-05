// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Binary_Search
// #Algorithm_II_Day_1_Binary_Search #Binary_Search_I_Day_11 #Level_2_Day_8_Binary_Search
// #Udemy_Binary_Search #Big_O_Time_O(log_n)_Space_O(1)
// #2024_09_05_Time_0_ms_(100.00%)_Space_2.1_MB_(99.00%)

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() as i32 - 1;

        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1); // Calculate mid index

            if nums[mid as usize] == target {
                return mid; // Target found, return the index
            }

            // If the left half is sorted
            if nums[lo as usize] <= nums[mid as usize] {
                // Check if the target lies within the sorted left half
                if nums[lo as usize] <= target && target <= nums[mid as usize] {
                    hi = mid - 1; // Target is in the left half
                } else {
                    lo = mid + 1; // Move to the right half
                }
            } else {
                // If the right half is sorted
                if nums[mid as usize] <= target && target <= nums[hi as usize] {
                    lo = mid + 1; // Target is in the right half
                } else {
                    hi = mid - 1; // Move to the left half
                }
            }
        }

        -1 // Target not found
    }
}

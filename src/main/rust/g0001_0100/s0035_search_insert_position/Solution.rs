// #Easy #Top_100_Liked_Questions #Array #Binary_Search #Algorithm_I_Day_1_Binary_Search
// #Binary_Search_I_Day_2 #Big_O_Time_O(log_n)_Space_O(1)
// #2024_09_05_Time_1_ms_(83.61%)_Space_2.1_MB_(70.02%)

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() as i32 - 1);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if target == nums[mid as usize] {
                return mid;
            } else if target < nums[mid as usize] {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

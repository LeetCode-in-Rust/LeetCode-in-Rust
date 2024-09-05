// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Binary_Search
// #Algorithm_II_Day_1_Binary_Search #Binary_Search_I_Day_5 #Big_O_Time_O(log_n)_Space_O(1)
// #2024_09_05_Time_0_ms_(100.00%)_Space_2.4_MB_(11.36%)

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![-1, -1];
        ans[0] = Solution::helper(&nums, target, false);
        ans[1] = Solution::helper(&nums, target, true);
        ans
    }

    fn helper(nums: &[i32], target: i32, equals: bool) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        let mut result = -1;
        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid as usize] == target {
                result = mid;
            }
            if nums[mid as usize] < target || (nums[mid as usize] == target && equals) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        result
    }
}

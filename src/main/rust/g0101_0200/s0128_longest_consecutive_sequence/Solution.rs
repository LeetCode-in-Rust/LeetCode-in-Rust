// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Hash_Table #Union_Find
// #Big_O_Time_O(N_log_N)_Space_O(1) #2024_09_09_Time_4_ms_(99.44%)_Space_3.4_MB_(62.78%)

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut nums = nums;
        nums.sort_unstable(); // Equivalent to Arrays.sort in Java
        
        let mut max = i32::MIN;
        let mut ths_max = 1;

        for i in 0..nums.len() - 1 {
            if nums[i + 1] == nums[i] + 1 {
                ths_max += 1;
            } else if nums[i + 1] != nums[i] {
                max = max.max(ths_max);
                ths_max = 1;
            }
        }

        max.max(ths_max) // Return the maximum sequence length
    }
}

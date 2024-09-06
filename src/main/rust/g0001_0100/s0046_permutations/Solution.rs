// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Backtracking
// #Algorithm_I_Day_11_Recursion_Backtracking #Level_2_Day_20_Brute_Force/Backtracking
// #Udemy_Backtracking/Recursion #Big_O_Time_O(n*n!)_Space_O(n+n!)
// #2024_09_06_Time_1_ms_(83.08%)_Space_2.1_MB_(76.15%)

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }

        let mut final_result = Vec::new();
        let mut curr_result = Vec::new();
        let mut used = vec![false; nums.len()];

        Solution::permute_recur(&nums, &mut final_result, &mut curr_result, &mut used);
        final_result
    }

    fn permute_recur(
        nums: &[i32],
        final_result: &mut Vec<Vec<i32>>,
        curr_result: &mut Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        if curr_result.len() == nums.len() {
            final_result.push(curr_result.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }

            curr_result.push(nums[i]);
            used[i] = true;

            Solution::permute_recur(nums, final_result, curr_result, used);

            used[i] = false;
            curr_result.pop();
        }
    }
}

// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Math #Two_Pointers
// #Algorithm_I_Day_2_Two_Pointers #Udemy_Arrays #Big_O_Time_O(n)_Space_O(1)
// #2024_09_10_Time_3_ms_(96.69%)_Space_3.6_MB_(76.46%)

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut rotated = vec![0; nums.len()];
        let k_fixed = k as usize % nums.len();

        for (i, item) in nums.iter().enumerate() {
            match nums.get(i+k_fixed) {
                Some(n) => rotated[i+k_fixed] = *item,
                None => rotated[(i+k_fixed)-nums.len()] = *item,
            }
        }
        nums.copy_from_slice(&rotated)
    }
}

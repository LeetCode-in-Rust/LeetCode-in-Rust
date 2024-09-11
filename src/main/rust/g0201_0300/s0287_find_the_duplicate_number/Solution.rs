// #Medium #Top_100_Liked_Questions #Array #Binary_Search #Two_Pointers #Bit_Manipulation
// #Binary_Search_II_Day_5 #Big_O_Time_O(n)_Space_O(n)
// #2024_09_11_Time_4_ms_(98.95%)_Space_3.2_MB_(46.32%)

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut arr = vec![0; nums.len() + 1];
        for &num in &nums {
            arr[num as usize] += 1;
            if arr[num as usize] == 2 {
                return num;
            }
        }
        0
    }
}

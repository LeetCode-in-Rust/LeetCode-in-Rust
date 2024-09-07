// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Sorting #Two_Pointers
// #Data_Structure_II_Day_2_Array #Udemy_Arrays #Big_O_Time_O(n)_Space_O(1)
// #2024_09_07_Time_0_ms_(100.00%)_Space_2.2_MB_(32.87%)

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut zeroes = 0;
        let mut ones = 0;

        // First pass: Count and place all zeroes
        for i in 0..nums.len() {
            if nums[i] == 0 {
                nums[zeroes] = 0;
                zeroes += 1;
            } else if nums[i] == 1 {
                ones += 1;
            }
        }

        // Second pass: Place all ones
        for j in zeroes..(zeroes + ones) {
            nums[j] = 1;
        }

        // Third pass: Place all twos
        for k in (zeroes + ones)..nums.len() {
            nums[k] = 2;
        }
    }
}

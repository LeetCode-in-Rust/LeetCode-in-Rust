// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Greedy #Two_Pointers
// #Algorithm_II_Day_4_Two_Pointers #Big_O_Time_O(n)_Space_O(1)
// #2024_09_04_Time_4_ms_(95.02%)_Space_2.9_MB_(95.02%)

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() as i32 - 1;
        while left < right {
            let h_left = height[left as usize];
            let h_right = height[right as usize];

            if h_left < h_right {
                max_area = max_area.max(h_left * (right - left));
                left += 1;
            } else {
                max_area = max_area.max(h_right * (right - left));
                right -= 1;
            }
        }
        max_area
    }
}

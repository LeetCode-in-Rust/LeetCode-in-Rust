// #Hard #Top_100_Liked_Questions #Top_Interview_Questions #Array #Dynamic_Programming #Two_Pointers
// #Stack #Monotonic_Stack #Dynamic_Programming_I_Day_9 #Udemy_Two_Pointers
// #Big_O_Time_O(n)_Space_O(1) #2024_09_06_Time_0_ms_(100.00%)_Space_2.2_MB_(67.75%)

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() as i32 - 1);
        let mut res = 0;
        let mut lower_wall = 0;

        while l < r {
            let l_val = height[l as usize];
            let r_val = height[r as usize];

            if l_val < r_val {
                lower_wall = lower_wall.max(l_val);
                res += lower_wall - l_val;
                l += 1;
            } else {
                lower_wall = lower_wall.max(r_val);
                res += lower_wall - r_val;
                r -= 1;
            }
        }

        res
    }
}

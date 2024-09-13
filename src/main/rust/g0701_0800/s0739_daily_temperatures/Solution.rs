// #Medium #Top_100_Liked_Questions #Array #Stack #Monotonic_Stack #Programming_Skills_II_Day_6
// #Big_O_Time_O(n)_Space_O(n) #2024_09_13_Time_33_ms_(91.80%)_Space_3.4_MB_(99.45%)

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut sol = vec![0; temperatures.len()];
        for i in (0..temperatures.len() - 1).rev() {
            let mut j = i + 1;
            while j < temperatures.len() {
                if temperatures[i] < temperatures[j] {
                    sol[i] = (j - i) as i32;
                    break;
                } else if sol[j] == 0 {
                    break;
                }
                j += sol[j] as usize;
            }
        }
        sol
    }
}

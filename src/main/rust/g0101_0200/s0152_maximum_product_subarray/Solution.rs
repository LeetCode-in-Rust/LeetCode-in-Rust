// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Dynamic_Programming
// #Dynamic_Programming_I_Day_6 #Level_2_Day_13_Dynamic_Programming #Udemy_Dynamic_Programming
// #Big_O_Time_O(N)_Space_O(1) #2024_09_09_Time_1_ms_(71.23%)_Space_2.2_MB_(53.42%)

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.into_iter()
        .fold((1f64, 1f64, f64::MIN),
        |(prod1, prod2, res), x| {
            let prod1 = prod1 * x as f64;
            let prod2 = prod2 * x as f64;
            ((x as f64).max(prod1).max(prod2), 
            (x as f64).min(prod1).min(prod2), 
            res.max(x as f64).max(prod1).max(prod2))
        }).2 as i32
    }
}

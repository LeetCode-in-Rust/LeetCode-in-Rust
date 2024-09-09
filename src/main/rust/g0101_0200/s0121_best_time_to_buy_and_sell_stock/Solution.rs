// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #Array #Dynamic_Programming
// #Data_Structure_I_Day_3_Array #Dynamic_Programming_I_Day_7 #Level_1_Day_5_Greedy #Udemy_Arrays
// #Big_O_Time_O(N)_Space_O(1) #2024_09_09_Time_3_ms_(98.62%)_Space_3_MB_(9.92%)


impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = prices[0];

        for price in prices.iter().skip(1) {
            if *price > min_price {
                max_profit = max_profit.max(*price - min_price);
            } else {
                min_price = *price;
            }
        }

        max_profit
    }
}

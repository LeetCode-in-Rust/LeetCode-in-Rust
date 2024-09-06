// #Medium #Top_100_Liked_Questions #Array #Backtracking #Algorithm_II_Day_10_Recursion_Backtracking
// #Level_2_Day_20_Brute_Force/Backtracking #Udemy_Backtracking/Recursion
// #Big_O_Time_O(2^n)_Space_O(n+2^n) #2024_09_06_Time_1_ms_(86.63%)_Space_2.2_MB_(19.77%)

impl Solution {
    pub fn combination_sum(coins: Vec<i32>, amount: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut sub_list = Vec::new();
        Solution::combination_sum_rec(coins.len() as i32, &coins, amount, &mut sub_list, &mut ans);
        ans
    }

    fn combination_sum_rec(
        n: i32,
        coins: &[i32],
        amount: i32,
        sub_list: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if amount == 0 || n == 0 {
            if amount == 0 {
                ans.push(sub_list.clone());
            }
            return;
        }

        if amount - coins[n as usize - 1] >= 0 {
            sub_list.push(coins[n as usize - 1]);
            Solution::combination_sum_rec(n, coins, amount - coins[n as usize - 1], sub_list, ans);
            sub_list.pop();
        }

        Solution::combination_sum_rec(n - 1, coins, amount, sub_list, ans);
    }
}

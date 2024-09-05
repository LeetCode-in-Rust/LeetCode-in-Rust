// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Dynamic_Programming
// #Backtracking #Algorithm_II_Day_11_Recursion_Backtracking #Udemy_Backtracking/Recursion
// #Big_O_Time_O(2^n)_Space_O(n) #2024_09_05_Time_1_ms_(83.92%)_Space_2.3_MB_(62.75%)

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        Solution::generate(&mut current, &mut result, n, n);
        result
    }

    fn generate(current: &mut String, result: &mut Vec<String>, open: i32, close: i32) {
        if open == 0 && close == 0 {
            result.push(current.clone());
            return;
        }

        if open > 0 {
            current.push('(');
            Solution::generate(current, result, open - 1, close);
            current.pop();
        }

        if close > 0 && open < close {
            current.push(')');
            Solution::generate(current, result, open, close - 1);
            current.pop();
        }
    }
}

// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Hash_Table #Backtracking
// #Algorithm_II_Day_11_Recursion_Backtracking #Udemy_Backtracking/Recursion
// #Big_O_Time_O(4^n)_Space_O(n) #2024_09_04_Time_0_ms_(100.00%)_Space_2.2_MB_(45.71%)

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let letters = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"
        ];

        let mut ans = vec![];
        let mut sb = String::new();
        Solution::find_combinations(0, &digits, &letters, &mut sb, &mut ans);
        ans
    }

    fn find_combinations(
        start: usize,
        digits: &str,
        letters: &[&str],
        curr: &mut String,
        ans: &mut Vec<String>,
    ) {
        if curr.len() == digits.len() {
            ans.push(curr.clone());
            return;
        }

        for i in start..digits.len() {
            let n = digits.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            for ch in letters[n].chars() {
                curr.push(ch);
                Solution::find_combinations(i + 1, digits, letters, curr, ans);
                curr.pop();
            }
        }
    }
}

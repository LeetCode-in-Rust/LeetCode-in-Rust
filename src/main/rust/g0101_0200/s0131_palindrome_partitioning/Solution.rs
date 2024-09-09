// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Dynamic_Programming
// #Backtracking #Big_O_Time_O(N*2^N)_Space_O(2^N*N)
// #2024_09_09_Time_65_ms_(85.18%)_Space_20.6_MB_(68.52%)

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        let mut curr_arr = Vec::new();
        Self::backtracking(&mut res, &mut curr_arr, &s, 0);
        res
    }

    fn backtracking(res: &mut Vec<Vec<String>>, curr_arr: &mut Vec<String>, s: &str, start: usize) {
        if start == s.len() {
            res.push(curr_arr.clone());
            return;
        }
        for end in start..s.len() {
            if !Self::is_palindrome(s, start, end) {
                continue;
            }
            curr_arr.push(s[start..=end].to_string());
            Self::backtracking(res, curr_arr, s, end + 1);
            curr_arr.pop();
        }
    }

    fn is_palindrome(s: &str, mut start: usize, mut end: usize) -> bool {
        while start < end && s.as_bytes()[start] == s.as_bytes()[end] {
            start += 1;
            end -= 1;
        }
        start >= end
    }
}

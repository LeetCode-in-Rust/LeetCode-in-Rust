// #Medium #String #Dynamic_Programming #Big_O_Time_O(n^2)_Space_O(n)
// #2024_09_13_Time_0_ms_(100.00%)_Space_2.2_MB_(38.46%)

impl Solution {
    fn expand(s: &[char], mut l: i32, mut r: i32, res: &mut i32) {
        while l >= 0 && (r as usize) < s.len() {
            if s[l as usize] != s[r as usize] {
                return;
            } else {
                *res += 1;
                l -= 1;
                r += 1;
            }
        }
    }

    pub fn count_substrings(s: String) -> i32 {
        let a: Vec<char> = s.chars().collect();
        let mut res = 0;
        for i in 0..a.len() {
            Solution::expand(&a, i as i32, i as i32, &mut res);       // Odd length palindromes
            Solution::expand(&a, i as i32, (i + 1) as i32, &mut res); // Even length palindromes
        }
        res
    }
}

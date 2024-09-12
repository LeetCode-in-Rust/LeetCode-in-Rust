// #Medium #Top_100_Liked_Questions #String #Hash_Table #Sliding_Window
// #Algorithm_II_Day_5_Sliding_Window #Programming_Skills_II_Day_12
// #Level_1_Day_12_Sliding_Window/Two_Pointer #Big_O_Time_O(n+m)_Space_O(1)
// #2024_09_12_Time_2_ms_(89.29%)_Space_2.2_MB_(100.00%)

use std::collections::VecDeque;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut map = vec![0; 26];
        let p_len = p.len();
        let s_len = s.len();
        
        // Fill the map with the character frequencies of string `p`
        for ch in p.chars() {
            map[ch as usize - 'a' as usize] += 1;
        }

        let mut res = Vec::new();
        let mut j = 0;

        // Sliding window
        for i in 0..s_len {
            let idx = s.as_bytes()[i] as usize - 'a' as usize;
            map[idx] -= 1;

            // If window size exceeds p_len, remove the left character
            if i >= p_len {
                let left_idx = s.as_bytes()[j] as usize - 'a' as usize;
                map[left_idx] += 1;
                j += 1;
            }

            // Check if the window is an anagram of `p`
            if i >= p_len - 1 && map.iter().all(|&x| x == 0) {
                res.push(j as i32);
            }
        }

        res
    }
}

// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #String #Hash_Table #Sorting
// #Data_Structure_II_Day_8_String #Programming_Skills_II_Day_11 #Udemy_Strings
// #Big_O_Time_O(n*k_log_k)_Space_O(n) #2024_09_06_Time_0_ms_(100.00%)_Space_5.3_MB_(26.81%)

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.iter() {
            let mut ch: Vec<char> = s.chars().collect();
            ch.sort_unstable();
            let temp: String = ch.iter().collect();

            hm.entry(temp).or_insert_with(Vec::new).push(s.clone());
        }

        hm.into_values().collect()
    }
}

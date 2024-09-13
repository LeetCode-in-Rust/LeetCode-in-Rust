// #Medium #String #Hash_Table #Greedy #Two_Pointers #Data_Structure_II_Day_7_String
// #Big_O_Time_O(n)_Space_O(1) #2024_09_13_Time_1_ms_(97.06%)_Space_2.1_MB_(41.18%)

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let letters: Vec<char> = s.chars().collect();
        let mut result = Vec::new();
        let mut position = vec![0; 26];

        // Store the last occurrence of each letter in the string
        for (i, &ch) in letters.iter().enumerate() {
            position[(ch as usize) - ('a' as usize)] = i;
        }

        let mut i = 0;
        let mut prev = -1;
        let mut max = 0;

        // Iterate over the string to find partitions
        while i < letters.len() {
            let ch_pos = position[(letters[i] as usize) - ('a' as usize)];
            if ch_pos > max {
                max = ch_pos;
            }

            if i == max {
                result.push((i as i32) - (prev as i32));
                prev = i as i32;
            }

            i += 1;
        }

        result
    }
}

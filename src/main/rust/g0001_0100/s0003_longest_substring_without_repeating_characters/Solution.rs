// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Hash_Table #Sliding_Window
// #Algorithm_I_Day_6_Sliding_Window #Level_2_Day_14_Sliding_Window/Two_Pointer #Udemy_Strings
// #Big_O_Time_O(n)_Space_O(1) #2024_08_24_Time_0_ms_(100.00%)_Space_2.3_MB_(28.72%)

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // Array to store last seen indices of characters
        let mut last_indices = [-1; 256];
        let mut max_len = 0;
        let mut cur_len = 0;
        let mut start = 0;

        // Convert string to bytes to use indexing
        let bytes = s.as_bytes();
        for (i, &cur) in bytes.iter().enumerate() {
            // Cast byte to usize to use as an index
            let cur = cur as usize;

            if last_indices[cur] < start as i32 {
                last_indices[cur] = i as i32;
                cur_len += 1;
            } else {
                let last_index = last_indices[cur];
                start = (last_index + 1) as usize;
                cur_len = i - start + 1;
                last_indices[cur] = i as i32;
            }

            if cur_len > max_len {
                max_len = cur_len;
            }
        }

        max_len as i32
    }
}

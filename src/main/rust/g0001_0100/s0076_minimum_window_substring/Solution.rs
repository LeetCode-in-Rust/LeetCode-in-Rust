// #Hard #Top_100_Liked_Questions #Top_Interview_Questions #String #Hash_Table #Sliding_Window
// #Level_2_Day_14_Sliding_Window/Two_Pointer #Big_O_Time_O(s.length())_Space_O(1)
// #2024_09_07_Time_0_ms_(100.00%)_Space_2.4_MB_(32.56%)

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map = vec![0; 128];
        for ch in t.chars() {
            map[ch as usize] += 1;
        }

        let mut count = t.len();
        let (mut begin, mut end, mut head) = (0, 0, 0);
        let mut d = usize::MAX;

        let s_chars: Vec<char> = s.chars().collect();
        while end < s.len() {
            if map[s_chars[end] as usize] > 0 {
                count -= 1;
            }
            map[s_chars[end] as usize] -= 1;
            end += 1;

            while count == 0 {
                if end - begin < d {
                    d = end - begin;
                    head = begin;
                }
                if map[s_chars[begin] as usize] == 0 {
                    count += 1;
                }
                map[s_chars[begin] as usize] += 1;
                begin += 1;
            }
        }

        if d == usize::MAX {
            return "".to_string();
        }
        s_chars[head..head + d].iter().collect()
    }
}

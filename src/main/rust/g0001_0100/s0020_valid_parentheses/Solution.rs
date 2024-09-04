// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #String #Stack
// #Data_Structure_I_Day_9_Stack_Queue #Udemy_Strings #Big_O_Time_O(n)_Space_O(n)
// #2024_09_04_Time_0_ms_(100.00%)_Space_2.2_MB_(50.76%)

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.is_empty() || stack.pop() != Some('(') {
                        return false;
                    }
                }
                '}' => {
                    if stack.is_empty() || stack.pop() != Some('{') {
                        return false;
                    }
                }
                ']' => {
                    if stack.is_empty() || stack.pop() != Some('[') {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

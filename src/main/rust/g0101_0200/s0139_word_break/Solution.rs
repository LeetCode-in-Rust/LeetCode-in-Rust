// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Hash_Table
// #Dynamic_Programming #Trie #Memoization #Algorithm_II_Day_15_Dynamic_Programming
// #Dynamic_Programming_I_Day_9 #Udemy_Dynamic_Programming #Big_O_Time_O(M+max*N)_Space_O(M+N+max)
// #2024_09_09_Time_0_ms_(100.00%)_Space_2.2_MB_(77.19%)

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut state = vec![None::<bool>; s.len() + 1];
        word_break_0(&s, &word_dict, &mut state)
    }
}

fn word_break_0(s: &str, word_dict: &Vec<String>, state: &mut Vec<Option<bool>>) -> bool {
    if s.is_empty() {
        return true;
    }

    if let Some(result) = state[s.len()] {
        return result;
    }

    let result = word_dict
        .iter()
        .filter_map(|word| s.strip_prefix(word))
        .any(|new_s| word_break_0(new_s, word_dict, state));

    state[s.len()] = Some(result);

    result
}

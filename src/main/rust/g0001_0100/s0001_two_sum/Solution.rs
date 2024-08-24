// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #Array #Hash_Table
// #Data_Structure_I_Day_2_Array #Level_1_Day_13_Hashmap #Udemy_Arrays #Big_O_Time_O(n)_Space_O(n)
// #2024_08_24_Time_0_ms_(100.00%)_Space_2.4_MB_(23.16%)

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_map: HashMap<i32, usize> = HashMap::new();
        
        for (i, &num) in numbers.iter().enumerate() {
            let required_num = target - num;
            if let Some(&index) = index_map.get(&required_num) {
                return vec![index as i32, i as i32];
            }
            index_map.insert(num, i);
        }
        
        vec![-1, -1]
    }
}

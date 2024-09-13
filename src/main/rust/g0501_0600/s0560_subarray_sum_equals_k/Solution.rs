// #Medium #Top_100_Liked_Questions #Array #Hash_Table #Prefix_Sum #Data_Structure_II_Day_5_Array
// #Big_O_Time_O(n)_Space_O(n) #2024_09_13_Time_7_ms_(96.88%)_Space_2.8_MB_(25.00%)

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut temp_sum = 0;
        let mut ret = 0;
        let mut sum_count: HashMap<i32, i32> = HashMap::new();
        sum_count.insert(0, 1);
        
        for &num in nums.iter() {
            temp_sum += num;
            if let Some(&count) = sum_count.get(&(temp_sum - k)) {
                ret += count;
            }
            let entry = sum_count.entry(temp_sum).or_insert(0);
            *entry += 1;
        }
        
        ret
    }
}

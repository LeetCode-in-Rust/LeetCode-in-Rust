// #Medium #Top_100_Liked_Questions #Array #Hash_Table #Sorting #Heap_Priority_Queue #Counting
// #Divide_and_Conquer #Quickselect #Bucket_Sort #Data_Structure_II_Day_20_Heap_Priority_Queue
// #Big_O_Time_O(n*log(n))_Space_O(k) #2024_09_12_Time_0_ms_(100.00%)_Space_2.4_MB_(66.00%)

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.iter().fold(HashMap::new(), |mut map, n| {
            let mut counter = map.entry(n).or_insert(0);
            *counter += 1;
            map
        })
        .drain()
        .fold(vec![(0, 0); k as usize], |mut top_k, (&num, count)| {
            if count > top_k[0].1 {
                top_k[0] = (num, count);

                let mut next_index = 1;
                while next_index < k as usize {
                    if count > top_k[next_index].1 {
                        let temp = top_k[next_index];
                        top_k[next_index] = (num, count);
                        top_k[next_index - 1] = temp;
                    }
                    next_index += 1;
                }
            }

            top_k
        })
        .into_iter()
        .map(|(num, count)| num)
        .collect()
    }
}

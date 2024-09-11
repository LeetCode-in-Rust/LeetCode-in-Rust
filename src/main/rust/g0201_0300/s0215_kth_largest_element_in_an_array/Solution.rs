// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Sorting #Heap_Priority_Queue
// #Divide_and_Conquer #Quickselect #Data_Structure_II_Day_20_Heap_Priority_Queue
// #Big_O_Time_O(n*log(n))_Space_O(log(n)) #2024_09_10_Time_10_ms_(90.24%)_Space_3.2_MB_(22.76%)

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        const NEGATIVE_OFFSET: usize = 10_000;

        let mut freq_array = [0; 20_001];
        for n in nums {
            freq_array[n as usize + NEGATIVE_OFFSET] += 1;
        }

        for i in (0..freq_array.len()).rev() {
            k -= freq_array[i];
            if k <= 0 {
                return (i - NEGATIVE_OFFSET) as i32;
            }
        }

        0
    }
}

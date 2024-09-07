// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Sorting
// #Data_Structure_II_Day_2_Array #Level_2_Day_17_Interval #Udemy_2D_Arrays/Matrix
// #Big_O_Time_O(n_log_n)_Space_O(n) #2024_09_07_Time_3_ms_(90.96%)_Space_2.9_MB_(71.81%)

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Sort the intervals by the start value
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut merged: Vec<Vec<i32>> = Vec::new();
        let mut current = intervals[0].clone();
        merged.push(current.clone());

        for next in intervals.into_iter().skip(1) {
            if current[1] >= next[0] {
                current[1] = current[1].max(next[1]);
                if let Some(last) = merged.last_mut() {
                    last[1] = current[1];
                }
            } else {
                current = next.clone();
                merged.push(current.clone());
            }
        }

        merged
    }
}

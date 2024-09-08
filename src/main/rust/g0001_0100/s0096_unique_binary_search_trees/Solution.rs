// #Medium #Dynamic_Programming #Math #Tree #Binary_Tree #Binary_Search_Tree
// #Dynamic_Programming_I_Day_11 #Big_O_Time_O(n)_Space_O(1)
// #2024_09_08_Time_0_ms_(100.00%)_Space_2.1_MB_(20.00%)

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut result: i64 = 1;
        for i in 0..n {
            result *= 2 * (n as i64) - (i as i64);
            result /= (i + 1) as i64;
        }
        result /= (n + 1) as i64;
        result as i32
    }
}

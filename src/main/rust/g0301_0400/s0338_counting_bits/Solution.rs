// #Easy #Dynamic_Programming #Bit_Manipulation #Udemy_Bit_Manipulation
// #Big_O_Time_O(num)_Space_O(num) #2024_09_12_Time_0_ms_(100.00%)_Space_2.6_MB_(15.06%)

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut result = vec![0; (num + 1) as usize];
        let mut border_pos = 1;
        let mut incr_pos = 1;
        
        for i in 1..=num {
            // When we reach a power of 2, reset border_pos and incr_pos
            if incr_pos == border_pos {
                result[i as usize] = 1;
                incr_pos = 1;
                border_pos = i;
            } else {
                result[i as usize] = 1 + result[incr_pos as usize];
                incr_pos += 1;
            }
        }
        
        result
    }
}

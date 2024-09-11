// #Medium #Top_100_Liked_Questions #Array #Binary_Search #Matrix #Divide_and_Conquer
// #Data_Structure_II_Day_4_Array #Binary_Search_II_Day_8 #Big_O_Time_O(n+m)_Space_O(1)
// #2024_09_11_Time_0_ms_(100.00%)_Space_2.6_MB_(75.76%)

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let mut r = 0;
        let mut c = matrix[0].len() as i32 - 1;
        
        while r < matrix.len() && c >= 0 {
            let val = matrix[r][c as usize];
            if val == target {
                return true;
            } else if val > target {
                c -= 1;
            } else {
                r += 1;
            }
        }
        
        false
    }
}

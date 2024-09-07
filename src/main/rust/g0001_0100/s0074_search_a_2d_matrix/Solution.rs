// #Medium #Top_100_Liked_Questions #Array #Binary_Search #Matrix #Data_Structure_I_Day_5_Array
// #Algorithm_II_Day_1_Binary_Search #Binary_Search_I_Day_8 #Level_2_Day_8_Binary_Search
// #Udemy_2D_Arrays/Matrix #Big_O_Time_O(endRow+endCol)_Space_O(1)
// #2024_09_07_Time_1_ms_(79.35%)_Space_2.3_MB_(12.90%)

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let end_row = matrix.len();
        let end_col = matrix[0].len();
        let mut target_row = 0;
        let mut result = false;

        // Find the row where the target could be present
        for i in 0..end_row {
            if matrix[i][end_col - 1] >= target {
                target_row = i;
                break;
            }
        }

        // Search for the target in the identified row
        for i in 0..end_col {
            if matrix[target_row][i] == target {
                result = true;
                break;
            }
        }

        result
    }
}

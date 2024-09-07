// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Hash_Table #Matrix
// #Udemy_2D_Arrays/Matrix #Big_O_Time_O(m*n)_Space_O(1)
// #2024_09_07_Time_0_ms_(100.00%)_Space_2.4_MB_(39.22%)

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row0 = false;
        let mut col0 = false;

        // Check if 0th column needs to be marked all 0s in future
        for row in matrix.iter() {
            if row[0] == 0 {
                col0 = true;
                break;
            }
        }

        // Check if 0th row needs to be marked all 0s in future
        for j in 0..n {
            if matrix[0][j] == 0 {
                row0 = true;
                break;
            }
        }

        // Store the signals in 0th row and column
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // Mark 0 for all cells based on signal from 0th row and 0th column
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // Set 0th column
        if col0 {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }

        // Set 0th row
        if row0 {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}

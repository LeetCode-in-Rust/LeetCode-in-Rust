// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Math #Matrix
// #Data_Structure_II_Day_3_Array #Programming_Skills_II_Day_7 #Udemy_2D_Arrays/Matrix
// #Big_O_Time_O(n^2)_Space_O(1) #2024_09_06_Time_0_ms_(100.00%)_Space_2.2_MB_(50.90%)

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in i..n - i - 1 {
                let pos = [
                    (i, j),
                    (j, n - 1 - i),
                    (n - 1 - i, n - 1 - j),
                    (n - 1 - j, i),
                ];

                let mut t = matrix[pos[0].0][pos[0].1];
                for k in 1..pos.len() {
                    let temp = matrix[pos[k].0][pos[k].1];
                    matrix[pos[k].0][pos[k].1] = t;
                    t = temp;
                }
                matrix[pos[0].0][pos[0].1] = t;
            }
        }
    }
}

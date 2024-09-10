// #Medium #Array #Dynamic_Programming #Matrix #Dynamic_Programming_I_Day_16
// #Big_O_Time_O(m*n)_Space_O(m*n) #2024_09_10_Time_16_ms_(88.89%)_Space_9.3_MB_(81.48%)

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        if n == 0 {
            return 0;
        }

        // Create a 2D dp vector initialized to 0
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut max_side = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    // Calculate the size of the square ending at (i, j)
                    dp[i + 1][j + 1] = 1 + dp[i][j].min(dp[i + 1][j]).min(dp[i][j + 1]);
                    max_side = max_side.max(dp[i + 1][j + 1]);
                }
            }
        }

        // Return the area of the largest square
        (max_side * max_side) as i32
    }
}

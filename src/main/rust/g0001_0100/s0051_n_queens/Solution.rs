// #Hard #Top_100_Liked_Questions #Array #Backtracking #Big_O_Time_O(N!)_Space_O(N)
// #2024_09_06_Time_1_ms_(90.82%)_Space_2.4_MB_(33.67%)

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut pos = vec![false; n as usize + 2 * n as usize - 1 + 2 * n as usize - 1];
        let mut pos2 = vec![0; n as usize];
        let mut ans = Vec::new();
        Solution::helper(n as usize, 0, &mut pos, &mut pos2, &mut ans);
        ans
    }

    fn helper(
        n: usize,
        row: usize,
        pos: &mut Vec<bool>,
        pos2: &mut Vec<usize>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            Solution::construct(n, pos2, ans);
            return;
        }

        for i in 0..n {
            let index = n + 2 * n - 1 + n - 1 + i - row;
            if pos[i] || pos[n + i + row] || pos[index] {
                continue;
            }

            pos[i] = true;
            pos[n + i + row] = true;
            pos[index] = true;
            pos2[row] = i;

            Solution::helper(n, row + 1, pos, pos2, ans);

            pos[i] = false;
            pos[n + i + row] = false;
            pos[index] = false;
        }
    }

    fn construct(n: usize, pos2: &Vec<usize>, ans: &mut Vec<Vec<String>>) {
        let mut sol = Vec::new();
        for r in 0..n {
            let mut queen_row = vec!['.'; n];
            queen_row[pos2[r]] = 'Q';
            sol.push(queen_row.into_iter().collect());
        }
        ans.push(sol);
    }
}

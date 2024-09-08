// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Matrix #Backtracking
// #Algorithm_II_Day_11_Recursion_Backtracking #Big_O_Time_O(4^(m*n))_Space_O(m*n)
// #2024_09_08_Time_1_ms_(99.28%)_Space_2.1_MB_(93.53%)

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let mut word = Vec::from_iter(word.chars());
        let mut map = [0; 58]; //A-65 z-122
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                map[(board[i][j] as u8 - 65) as usize] += 1;
            }
        }

        for c in &word {
            if map[(*c as u8 - 65) as usize] == 0 {
                return false;
            }
        }

        if map[(word[0] as u8 - 65) as usize] > map[(word[word.len() - 1] as u8 - 65) as usize] {
            word.reverse();
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] != word[0] {
                    continue;
                }
                let mut p = 0;
                let (mut k, mut l) = (i, j);
                let mut dir = vec![-1; word.len() - 1];
                loop {
                    if p == word.len() - 1 {
                        return true;
                    }
                    if dir[p] == -1 {
                        board[k][l] = ' ';
                        dir[p] = 0;
                        if k > 0 && board[k - 1][l] == word[p + 1] {
                            p += 1;
                            k -= 1;
                            continue;
                        }
                    }
                    if dir[p] == 0 {
                        dir[p] = 1;
                        if l < board[k].len() - 1 && board[k][l + 1] == word[p + 1] {
                            p += 1;
                            l += 1;
                            continue;
                        }
                    }
                    if dir[p] == 1 {
                        dir[p] = 2;
                        if k < board.len() - 1 && board[k + 1][l] == word[p + 1] {
                            p += 1;
                            k += 1;
                            continue;
                        }
                    }
                    if dir[p] == 2 {
                        dir[p] = 3;
                        if l > 0 && board[k][l - 1] == word[p + 1] {
                            p += 1;
                            l -= 1;
                            continue;
                        }
                    }
                    while p > 0 && dir[p] == 3 {
                        board[k][l] = word[p];
                        dir[p] = -1;
                        p -= 1;
                        if dir[p] == 0 {
                            k += 1;
                            continue;
                        }
                        if dir[p] == 1 {
                            l -= 1;
                            continue;
                        }
                        if dir[p] == 2 {
                            k -= 1;
                            continue;
                        }
                        if dir[p] == 3 {
                            l += 1;
                            continue;
                        }
                    }
                    if p == 0 && dir[p] == 3 {
                        board[k][l] = word[p];
                        break;
                    }
                }
            }
        }
        false
    }
}

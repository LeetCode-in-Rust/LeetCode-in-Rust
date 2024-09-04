// #Hard #Top_Interview_Questions #String #Dynamic_Programming #Recursion #Udemy_Dynamic_Programming
// #Big_O_Time_O(m*n)_Space_O(m*n) #2024_09_04_Time_0_ms_(100.00%)_Space_2.1_MB_(25.55%)

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        is_match_dp(s.as_bytes(), p.as_bytes())
    }
}

pub fn is_match_dp(s: &[u8], p: &[u8]) -> bool {
    let mut grid = vec![vec![false; s.len() + 1]; p.len() + 1];
    grid[p.len()][s.len()] = true;

    let mut pi = p.len();
    while pi > 0 {
        pi -= 1;

        let mstar = pi > 0 && p[pi] == b'*';
        if mstar {
            pi -= 1;
        }

        for si in 0..=s.len() {
            let is_match = if mstar {
                let mut si2 = si;
                loop {
                    if grid[pi + 2][si2] {
                        break true;
                    }
                    if si2 < s.len() && (p[pi] == b'.' || s[si2] == p[pi]) {
                        si2 += 1;
                        continue;
                    }
                    break false;
                }
            } else {
                si < s.len() && (p[pi] == b'.' || s[si] == p[pi]) && grid[pi + 1][si + 1]
            };
            grid[pi][si] = is_match;
        }
    }

    grid[0][0]
}

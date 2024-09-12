// #Medium #Top_100_Liked_Questions #Array #Dynamic_Programming #Binary_Search
// #Algorithm_II_Day_16_Dynamic_Programming #Binary_Search_II_Day_3 #Dynamic_Programming_I_Day_18
// #Udemy_Dynamic_Programming #Big_O_Time_O(n*log_n)_Space_O(n)
// #2024_09_12_Time_0_ms_(100.00%)_Space_2.1_MB_(100.00%)

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut dp = vec![i32::MAX; nums.len() + 1];
        let (mut left, mut right) = (1, 1);

        for &curr in nums.iter() {
            let (mut start, mut end) = (left, right);

            // Binary search to find the position to update
            while start + 1 < end {
                let mid = start + (end - start) / 2;
                if dp[mid as usize] > curr {
                    end = mid;
                } else {
                    start = mid;
                }
            }

            // Update the dp array
            if dp[start as usize] > curr {
                dp[start as usize] = curr;
            } else if curr > dp[start as usize] && curr < dp[end as usize] {
                dp[end as usize] = curr;
            } else if curr > dp[end as usize] {
                dp[end as usize + 1] = curr;
                right += 1;
            }
        }

        right
    }
}

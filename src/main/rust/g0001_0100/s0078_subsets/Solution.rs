// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Bit_Manipulation #Backtracking
// #Algorithm_II_Day_9_Recursion_Backtracking #Udemy_Backtracking/Recursion
// #Big_O_Time_O(2^n)_Space_O(n*2^n) #2024_09_07_Time_0_ms_(100.00%)_Space_2.2_MB_(74.79%)

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::solve(&nums, &mut vec![], &mut res, 0);
        res
    }

    fn solve(nums: &[i32], temp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start: usize) {
        res.push(temp.clone());
        for i in start..nums.len() {
            temp.push(nums[i]);
            Self::solve(nums, temp, res, i + 1);
            temp.pop();
        }
    }
}

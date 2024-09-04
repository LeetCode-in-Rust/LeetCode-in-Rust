// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Sorting #Two_Pointers
// #Data_Structure_II_Day_1_Array #Algorithm_II_Day_3_Two_Pointers #Udemy_Two_Pointers
// #Big_O_Time_O(n*log(n))_Space_O(n^2) #2024_09_04_Time_27_ms_(81.94%)_Space_4.1_MB_(55.03%)

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let len = nums.len();
        let mut result = Vec::new();

        for i in 0..len - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = len - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];

                if sum < 0 {
                    l += 1;
                } else if sum > 0 {
                    r -= 1;
                } else {
                    result.push(vec![nums[i], nums[l], nums[r]]);

                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }

                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }

                    l += 1;
                    r -= 1;
                }
            }
        }

        result
    }
}

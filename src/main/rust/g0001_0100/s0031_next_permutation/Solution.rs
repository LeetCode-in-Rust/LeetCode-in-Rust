// #Medium #Top_100_Liked_Questions #Array #Two_Pointers #Big_O_Time_O(n)_Space_O(1)
// #2024_09_05_Time_1_ms_(78.64%)_Space_2.2_MB_(31.07%)

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }

        // Step 1: Find the first decreasing element from the end
        let mut i = nums.len() as i32 - 2;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }

        // Step 2: If the entire sequence isn't decreasing, swap the element
        if i >= 0 {
            let mut j = nums.len() as i32 - 1;
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            Solution::swap(nums, i as usize, j as usize);
        }

        // Step 3: Reverse the elements after the found element
        Solution::reverse(nums, (i + 1) as usize, nums.len() - 1);
    }

    // Swap elements at index i and j
    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
    }

    // Reverse the elements in the slice from index i to j
    fn reverse(nums: &mut Vec<i32>, mut i: usize, mut j: usize) {
        while i < j {
            Solution::swap(nums, i, j);
            i += 1;
            j -= 1;
        }
    }
}

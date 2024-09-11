// #Medium #Top_100_Liked_Questions #Array #Prefix_Sum #Data_Structure_II_Day_5_Array #Udemy_Arrays
// #Big_O_Time_O(n^2)_Space_O(n) #2024_09_11_Time_8_ms_(87.13%)_Space_3.2_MB_(89.17%)

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut prev = nums[0];
        let n = nums.len();
        
        // calc prefix
        res.push(1);
        for i in 1..n{
            let mut prod = 1;
            prod *= prev;
            prev *= nums[i];
            res.push(prod);
        }
        
        // calc suffix
        prev = nums[n - 1];
        for i in (0..n - 1).rev(){
            let mut prod = 1;
            prod *= prev;
            prev *= nums[i];
            res[i] *= prod;
        }

        res
    }
}

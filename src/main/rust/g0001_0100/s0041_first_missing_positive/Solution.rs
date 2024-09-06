// #Hard #Top_100_Liked_Questions #Top_Interview_Questions #Array #Hash_Table #Udemy_Arrays
// #Big_O_Time_O(n)_Space_O(n) #2024_09_06_Time_3_ms_(97.44%)_Space_2.9_MB_(100.00%)

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut max = nums.len()+1;
        let mut vec:Vec<i32> = vec![0;max+2];
        for n in nums {
            if n <= 0 {
                max -= 1;
            } else if n <= (max as i32) {
                vec[n as usize] += 1;
            }
        }
        for i in 1..(max+1) {
            if vec[i] == 0 { return i as i32; }
        }
        return 1;
    }
}

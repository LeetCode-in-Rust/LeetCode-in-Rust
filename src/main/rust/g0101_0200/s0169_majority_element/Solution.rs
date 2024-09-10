// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #Array #Hash_Table #Sorting #Counting
// #Divide_and_Conquer #Data_Structure_II_Day_1_Array #Udemy_Famous_Algorithm
// #Big_O_Time_O(n)_Space_O(1) #2024_09_10_Time_1_ms_(82.64%)_Space_2.5_MB_(18.91%)

impl Solution {
    pub fn majority_element(arr: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut majority = arr[0];

        // For Potential Majority Element
        for &num in arr.iter().skip(1) {
            if num == majority {
                count += 1;
            } else {
                if count > 1 {
                    count -= 1;
                } else {
                    majority = num;
                }
            }
        }

        // For Confirmation
        count = 0;
        for &num in arr.iter() {
            if num == majority {
                count += 1;
            }
        }

        if count >= (arr.len() / 2) + 1 {
            majority
        } else {
            -1
        }
    }
}

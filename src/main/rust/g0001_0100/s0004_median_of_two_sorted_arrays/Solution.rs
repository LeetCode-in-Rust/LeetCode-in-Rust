// #Hard #Top_100_Liked_Questions #Top_Interview_Questions #Array #Binary_Search #Divide_and_Conquer
// #Big_O_Time_O(log(min(N,M)))_Space_O(1) #2024_08_24_Time_0_ms_(100.00%)_Space_2.2_MB_(39.80%)

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            // Ensures nums1 is the smaller array
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        
        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut low = 0;
        let mut high = n1;

        while low <= high {
            let cut1 = (low + high) / 2;
            let cut2 = (n1 + n2 + 1) / 2 - cut1;

            let l1 = if cut1 == 0 { i32::MIN } else { nums1[cut1 - 1] };
            let l2 = if cut2 == 0 { i32::MIN } else { nums2[cut2 - 1] };
            let r1 = if cut1 == n1 { i32::MAX } else { nums1[cut1] };
            let r2 = if cut2 == n2 { i32::MAX } else { nums2[cut2] };

            if l1 <= r2 && l2 <= r1 {
                // Found the correct partition
                if (n1 + n2) % 2 == 0 {
                    return (f64::from(l1.max(l2)) + f64::from(r1.min(r2))) / 2.0;
                } else {
                    return f64::from(l1.max(l2));
                }
            } else if l1 > r2 {
                high = cut1 - 1;
            } else {
                low = cut1 + 1;
            }
        }

        // Fallback case, should never hit
        0.0
    }
}

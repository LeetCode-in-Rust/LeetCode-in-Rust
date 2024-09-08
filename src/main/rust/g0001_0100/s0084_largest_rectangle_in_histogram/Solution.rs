// #Hard #Top_100_Liked_Questions #Top_Interview_Questions #Array #Stack #Monotonic_Stack
// #Big_O_Time_O(n_log_n)_Space_O(log_n) #2024_09_08_Time_7_ms_(92.48%)_Space_3.2_MB_(96.24%)

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        Self::largest_area(&heights, 0, heights.len())
    }

    fn largest_area(heights: &[i32], start: usize, limit: usize) -> i32 {
        if heights.is_empty() || start == limit {
            return 0;
        }
        if limit - start == 1 {
            return heights[start];
        }
        if limit - start == 2 {
            let max_of_two_bars = heights[start].max(heights[start + 1]);
            let area_from_two = heights[start].min(heights[start + 1]) * 2;
            return max_of_two_bars.max(area_from_two);
        }
        if Self::check_if_sorted(heights, start, limit) {
            let mut max_when_sorted = 0;
            for i in start..limit {
                max_when_sorted = max_when_sorted.max(heights[i] * (limit - i) as i32);
            }
            return max_when_sorted;
        } else {
            let min_ind = Self::find_min_in_array(heights, start, limit);
            return Self::max_of_three_nums(
                Self::largest_area(heights, start, min_ind),
                heights[min_ind] * (limit - start) as i32,
                Self::largest_area(heights, min_ind + 1, limit),
            );
        }
    }

    fn find_min_in_array(heights: &[i32], start: usize, limit: usize) -> usize {
        let mut min = i32::MAX;
        let mut min_index = 0;
        for i in start..limit {
            if heights[i] < min {
                min = heights[i];
                min_index = i;
            }
        }
        min_index
    }

    fn check_if_sorted(heights: &[i32], start: usize, limit: usize) -> bool {
        for i in start + 1..limit {
            if heights[i] < heights[i - 1] {
                return false;
            }
        }
        true
    }

    fn max_of_three_nums(a: i32, b: i32, c: i32) -> i32 {
        a.max(b).max(c)
    }
}

// #Hard #Top_100_Liked_Questions #Sorting #Two_Pointers #Design #Heap_Priority_Queue #Data_Stream
// #Big_O_Time_O(n*log_n)_Space_O(n) #2024_09_11_Time_58_ms_(99.02%)_Space_23.2_MB_(84.31%)

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,                // Max heap for lower half
    min_heap: BinaryHeap<Reverse<i32>>,       // Min heap for upper half
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MedianFinder {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),                // Max heap (lower half)
            min_heap: BinaryHeap::new(),                // Min heap (upper half), with Reverse for min behavior
        }
    }

    /** Add a number to the data structure */
    pub fn add_num(&mut self, num: i32) {
        if self.max_heap.is_empty() || *self.max_heap.peek().unwrap() > num {
            self.max_heap.push(num);  // Add to the max heap (lower half)
        } else {
            self.min_heap.push(Reverse(num));  // Add to the min heap (upper half)
        }

        // Balance the two heaps
        if (self.max_heap.len() as i32 - self.min_heap.len() as i32).abs() > 1 {
            self.balance();
        }
    }

    /** Balance the heaps */
    fn balance(&mut self) {
        if self.max_heap.len() > self.min_heap.len() {
            self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        } else {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    /** Find the median of the numbers added so far */
    pub fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64) / 2.0
        } else if self.max_heap.len() > self.min_heap.len() {
            *self.max_heap.peek().unwrap() as f64
        } else {
            self.min_heap.peek().unwrap().0 as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

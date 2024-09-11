// #Hard #Top_100_Liked_Questions #Array #Heap_Priority_Queue #Sliding_Window #Queue
// #Monotonic_Queue #Udemy_Arrays #Big_O_Time_O(n*k)_Space_O(n+k)
// #2024_09_11_Time_43_ms_(84.62%)_Space_3.7_MB_(48.72%)

use std::collections::VecDeque;

struct MaxQueue {
    queue: VecDeque<i32>,
}

impl MaxQueue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, element: i32) {
        while !self.queue.is_empty() && *self.queue.back().unwrap() < element {
            self.queue.pop_back();
        }
        self.queue.push_back(element);
    }

    fn dequeue(&mut self, removed_element: i32) -> i32 {
        if !self.queue.is_empty() && *self.queue.front().unwrap() == removed_element {
            self.queue.pop_front().unwrap()
        } else {
            removed_element
        }
    }

    // Get the maximum element in the queue
    fn get_max(&self) -> Option<i32> {
        self.queue.front().copied()
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut max_queue = MaxQueue::new();
        let mut result: Vec<i32> = vec![0; n - k + 1];

        // Fill initial window in the MaxQueue
        for i in 0..k {
            max_queue.enqueue(nums[i]);
        }

        // Calculate maximum for each window
        for i in k..n {
            // Get maximum from MaxQueue
            result[i - k] = max_queue.get_max().unwrap();

            // Move window by dequeuing the element leaving the window
            max_queue.dequeue(nums[i - k]);

            // Add the current element to the MaxQueue
            max_queue.enqueue(nums[i]);
        }

        // Get the maximum for the last window
        result[n - k] = max_queue.get_max().unwrap();

        result
    }
}

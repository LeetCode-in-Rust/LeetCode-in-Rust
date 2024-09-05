// #Hard #Top_100_Liked_Questions #Top_Interview_Questions #Heap_Priority_Queue #Linked_List
// #Divide_and_Conquer #Merge_Sort #Big_O_Time_O(k*n*log(k))_Space_O(log(k))
// #2024_09_05_Time_0_ms_(100.00%)_Space_3.2_MB_(74.80%)

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct HeapNode(Box<ListNode>);

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.val == other.0.val
    }
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.val.cmp(&self.0.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        
        for list in lists {
            if let Some(node) = list {
                heap.push(HeapNode(node));
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let Some(HeapNode(mut node)) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(HeapNode(next));
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

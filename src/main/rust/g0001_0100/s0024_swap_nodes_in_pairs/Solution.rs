// #Medium #Top_100_Liked_Questions #Linked_List #Recursion #Data_Structure_II_Day_12_Linked_List
// #Udemy_Linked_List #Big_O_Time_O(n)_Space_O(1)
// #2024_09_05_Time_0_ms_(100.00%)_Space_2.1_MB_(69.74%)

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
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode {
            val: 0,
            next: head,
        };
        let mut current = &mut dummy;
        
        while let Some(mut first) = current.next.take() {
            if let Some(mut second) = first.next.take() {
                first.next = second.next.take();
                second.next = Some(first);
                current.next = Some(second);
                current = current.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                current.next = Some(first);
                break;
            }
        }      

        dummy.next   
    }
}

// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #Linked_List #Recursion
// #Data_Structure_I_Day_7_Linked_List #Algorithm_I_Day_10_Recursion_Backtracking
// #Level_1_Day_3_Linked_List #Udemy_Linked_List #Big_O_Time_O(m+n)_Space_O(m+n)
// #2024_09_05_Time_0_ms_(100.00%)_Space_2.1_MB_(14.29%)

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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(-1)));
        let mut current = &mut dummy;

        while l1.is_some() || l2.is_some() {
            if let (Some(ref l1_node), Some(ref l2_node)) = (&l1, &l2) {
                if l1_node.val <= l2_node.val {
                    current.as_mut().unwrap().next = l1.take();
                    l1 = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
                } else {
                    current.as_mut().unwrap().next = l2.take();
                    l2 = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
                }
            } else if l1.is_some() {
                current.as_mut().unwrap().next = l1.take();
            } else {
                current.as_mut().unwrap().next = l2.take();
            }
            current = &mut current.as_mut().unwrap().next;
        }

        dummy.unwrap().next
    }
}

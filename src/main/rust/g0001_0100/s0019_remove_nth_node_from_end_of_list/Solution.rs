// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Two_Pointers #Linked_List
// #Algorithm_I_Day_5_Two_Pointers #Level_2_Day_3_Linked_List #Big_O_Time_O(L)_Space_O(L)
// #2024_09_04_Time_0_ms_(100.00%)_Space_2.1_MB_(58.02%)

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Use a naive solution
        // Get the length of the list
        // Traverse again up to target - 1, remove target and point next to the next item
        let mut length: i32 = 0;
        let mut node: &Option<Box<ListNode>> = &head;
        while let Some(node_val) = node {
            length += 1;
            node = &node_val.next;
        }

        // Create a new list to contain the result with node removed from it
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut current = result.as_mut();
        let mut node = &head;
        let mut x: i32 = 1;

        while let Some(node_val) = node {
            if (length - x + 1) != n {
                current.as_mut().unwrap().next = Some(Box::new(ListNode::new(node_val.val)));
                current = current.unwrap().next.as_mut();
            }
            x += 1;
            node = &node_val.next;
        }

        result.unwrap().next
    }
}

// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Math #Linked_List #Recursion
// #Data_Structure_II_Day_10_Linked_List #Programming_Skills_II_Day_15
// #Big_O_Time_O(max(N,M))_Space_O(max(N,M)) #2024_08_24_Time_0_ms_(100.00%)_Space_2.2_MB_(14.25%)

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>, 
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut p = l1;
        let mut q = l2;
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut curr = &mut dummy_head;
        let mut carry = 0;

        while p.is_some() || q.is_some() {
            let x = p.as_ref().map_or(0, |node| node.val);
            let y = q.as_ref().map_or(0, |node| node.val);
            let sum = carry + x + y;
            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
            
            if let Some(node) = p {
                p = node.next;
            }
            if let Some(node) = q {
                q = node.next;
            }
        }

        if carry > 0 {
            curr.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy_head.next
    }
}

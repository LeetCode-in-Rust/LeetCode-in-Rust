// #Easy #Top_100_Liked_Questions #Two_Pointers #Stack #Linked_List #Recursion
// #Level_2_Day_3_Linked_List #Udemy_Linked_List #Big_O_Time_O(n)_Space_O(1)
// #2024_09_11_Time_43_ms_(85.29%)_Space_9.2_MB_(34.31%)

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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut ref_node = &head;
        let mut len = 0;

        while let Some(ref node) = &ref_node {
            ref_node = &node.next;
            len += 1;
        }

        ref_node = &head;
        let mut i = 0;
        let mut num = 0;
        let mut skip_num = if len % 2 == 1 { len / 2 + 1 } else { 0 };

        while let Some(node) = ref_node {
            ref_node = &node.next;

            if skip_num > 0 && skip_num - 1 == i {
                skip_num = 0;
                continue;
            }

            if i < len / 2 {
                num = num * 10 + node.val as usize;
            } else {
                num -= node.val as usize * 10_usize.pow(i - len / 2);
            }
            i += 1;
        }

        num == 0
    }
}

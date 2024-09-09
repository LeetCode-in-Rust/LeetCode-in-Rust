// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Sorting #Two_Pointers #Linked_List
// #Divide_and_Conquer #Merge_Sort #Level_2_Day_4_Linked_List #Big_O_Time_O(log(N))_Space_O(log(N))
// #2024_09_09_Time_21_ms_(81.82%)_Space_6.3_MB_(22.73%)

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec: Vec<i32> = vec![];
        while let Some(mut node) = head { 
            vec.push(node.val);
            head = node.next;
        } 
        vec.sort(); 
        let mut next = None; 
        while let Some(val) = vec.pop() { 
            next = Some(Box::new(ListNode{next, val}))
        }
        next 
    }
}

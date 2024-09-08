// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Breadth_First_Search #Tree
// #Binary_Tree #Data_Structure_I_Day_11_Tree #Level_1_Day_6_Tree #Udemy_Tree_Stack_Queue
// #Big_O_Time_O(N)_Space_O(N) #2024_09_08_Time_1_ms_(80.61%)_Space_2.4_MB_(24.49%)

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q = VecDeque::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        q.push_back(vec![root]);
        while let Some(cur_level) = q.pop_front() {
            let mut cur_level_vals = Vec::new();
            let mut children = Vec::new();
            for node_option in cur_level {
                if let Some(node) = node_option {
                    cur_level_vals.push(node.borrow().val);
                    children.push(node.borrow().left.clone());
                    children.push(node.borrow().right.clone());
                }
            }
            if cur_level_vals.len() > 0 {
                ans.push(cur_level_vals);
                q.push_back(children);
            }
        }
        return ans;
    }
}

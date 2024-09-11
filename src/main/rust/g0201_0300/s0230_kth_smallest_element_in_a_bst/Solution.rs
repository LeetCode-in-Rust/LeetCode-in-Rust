// #Medium #Top_100_Liked_Questions #Depth_First_Search #Tree #Binary_Tree #Binary_Search_Tree
// #Data_Structure_II_Day_17_Tree #Level_2_Day_9_Binary_Search_Tree #Big_O_Time_O(n)_Space_O(n)
// #2024_09_11_Time_0_ms_(100.00%)_Space_3.1_MB_(15.07%)

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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut vec = vec![];
        Self::in_order(&root, &mut vec);
        vec[k as usize - 1] 
    }

    pub fn in_order(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(node) = root {
            let _ = Self::in_order(&node.borrow().left, vec);
            vec.push(node.borrow().val);
            let _ = Self::in_order(&node.borrow().right, vec);
        }
    }
}

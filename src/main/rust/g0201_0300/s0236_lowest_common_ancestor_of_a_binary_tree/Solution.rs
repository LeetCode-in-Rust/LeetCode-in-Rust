// #Medium #Top_100_Liked_Questions #Depth_First_Search #Tree #Binary_Tree
// #Data_Structure_II_Day_18_Tree #Udemy_Tree_Stack_Queue #Big_O_Time_O(n)_Space_O(n)
// #2024_09_11_Time_0_ms_(100.00%)_Space_4.2_MB_(31.48%)

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
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let (Some(rn), Some(pn), Some(qn)) = (&root, &p, &q) {
            
            if rn.borrow().val == pn.borrow().val || rn.borrow().val == qn.borrow().val {
                return root;
            }

            let left = Self::lowest_common_ancestor(
                rn.borrow().left.as_ref().map(Rc::clone),
                Some(Rc::clone(pn)),
                Some(Rc::clone(qn)),
            );

            let right = Self::lowest_common_ancestor(
                rn.borrow().right.as_ref().map(Rc::clone),
                Some(Rc::clone(pn)),
                Some(Rc::clone(qn)),
            );

            if left.is_some() && right.is_some() {
                return root;
            }

            if left.is_some() {
                return left;
            }

            return right;
        }
        None    
    }
}

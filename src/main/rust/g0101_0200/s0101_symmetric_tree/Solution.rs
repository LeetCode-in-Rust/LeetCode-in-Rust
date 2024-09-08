// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #Depth_First_Search #Breadth_First_Search
// #Tree #Binary_Tree #Data_Structure_I_Day_11_Tree #Level_2_Day_15_Tree
// #Big_O_Time_O(N)_Space_O(log(N)) #2024_09_08_Time_0_ms_(100.00%)_Space_2.2_MB_(78.98%)

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn compare(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (l, r) {
                (None, None) => true,
                (None, Some(n)) | (Some(n), None) => false,
                (Some(l), Some(r)) => {
                    if l.borrow().val != r.borrow().val {
                        return false;
                    }
                    return compare(l.borrow().left.clone(), r.borrow().right.clone())
                        && compare(l.borrow().right.clone(), r.borrow().left.clone())
                }
            }
        }
        match root {
            Some(r) => compare(r.borrow().left.clone(), r.borrow().right.clone()),
            None => true
        }
    }
}

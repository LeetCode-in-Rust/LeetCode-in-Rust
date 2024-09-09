// #Hard #Top_100_Liked_Questions #Top_Interview_Questions #Dynamic_Programming #Depth_First_Search
// #Tree #Binary_Tree #Udemy_Tree_Stack_Queue #Big_O_Time_O(N)_Space_O(N)
// #2024_09_09_Time_0_ms_(100.00%)_Space_4.3_MB_(90.79%)

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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = i32::MIN;
        Solution::helper(&root, &mut max);
        max
    }

    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(n) = node {
            let n = n.borrow();
            let left = Solution::helper(&n.left, max).max(0);
            let right = Solution::helper(&n.right, max).max(0);
            let current = n.val + left + right;
            *max = (*max).max(current);
            return n.val + left.max(right);
        }
        0
    }
}

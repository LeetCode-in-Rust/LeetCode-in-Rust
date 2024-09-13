// #Easy #Top_100_Liked_Questions #Depth_First_Search #Tree #Binary_Tree #Level_2_Day_7_Tree
// #Udemy_Tree_Stack_Queue #Big_O_Time_O(n)_Space_O(n)
// #2024_09_13_Time_0_ms_(100.00%)_Space_3.9_MB_(13.76%)

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
use std::cmp::max;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;

        fn helper(root: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> (i32, i32) {
            if let Some(root) = root {
                let (l1, l2) = helper(root.borrow().left.clone(), diameter);
                let (r1, r2) = helper(root.borrow().right.clone(), diameter);

                *diameter = max(*diameter, l1 + l2);
                *diameter = max(*diameter, r1 + r2);

                let l = l1.max(l2) + 1;
                let r = r1.max(r2) + 1;
                return (l, r);
            }

            // (depth, diameter that doesn't pass through )
            (-1, -1)
        }

        let (l, r) = helper(root, &mut diameter);
        diameter = max(diameter, l + r);
        diameter
    }
}

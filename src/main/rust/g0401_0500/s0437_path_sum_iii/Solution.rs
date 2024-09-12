// #Medium #Depth_First_Search #Tree #Binary_Tree #Level_2_Day_7_Tree #Big_O_Time_O(n)_Space_O(n)
// #2024_09_12_Time_0_ms_(100.00%)_Space_4.7_MB_(10.42%)

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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let target_sum = target_sum as i64;
        let mut stack = vec![(root, vec![])];
        let mut ans = 0;
        while let Some((node, mut path)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                path.push(node.val);
                ans += path
                    .iter()
                    .rev()
                    .fold((0, 0), |(res, acc), &x| {
                        let new_acc = acc + x as i64;
                        (
                            res + (new_acc == target_sum) as i32,
                            new_acc
                        )
                    })
                    .0;
                stack.push((node.left.clone(), path.clone()));
                stack.push((node.right.clone(), path.clone()));
            } else {
                continue;
            }
        }
        ans
    }
}

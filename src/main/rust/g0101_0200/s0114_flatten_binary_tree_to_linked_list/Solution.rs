// #Medium #Array #Hash_Table #Tree #Binary_Tree #Divide_and_Conquer
// #2024_09_08_Time_0_ms_(100.00%)_Space_2.7_MB_(50.00%)

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
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Node {
        Solution::builder(&inorder[..], &postorder[..])
    }
    fn builder(inorder: &[i32], postorder: &[i32]) -> Node { 
        let i = inorder.len();
        if inorder.is_empty() || postorder.is_empty() { 
            return None 
        } else { 
            let n = postorder.len() -1;
            let mut root = TreeNode::new(postorder[n]);
            let m = inorder.iter().position(|&x| x == postorder[n]).unwrap();
            root.left = Solution::builder(&inorder[0..m], &postorder[0..m]);
            root.right = Solution::builder(&inorder[m+1..i], &postorder[m..i-1]);
            Some(Rc::new(RefCell::new(root)))
        }
    }
}
